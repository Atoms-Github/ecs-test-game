use std::borrow::{Borrow, BorrowMut, Cow};
use std::collections::HashMap;

use bincode::Deserializer;
use bincode::ErrorKind::SizeLimit;
use duckdb::arrow::compute::filter;
use ggez::graphics::Color;
use ggez::input::mouse::position;
use ggez::winit::dpi::Position;
use glam::*;
use legion::serialize::{set_entity_serializer, Canon};
use legion::storage::ComponentStorage;
use legion::systems::CommandBuffer;
use legion::world::{ComponentAccess, EntityAccessError, EntryRef};
use legion::*;
use rand::Rng;
use serde::de::DeserializeSeed;

use crate::brains::com::*;
use crate::brains::{Brain, SystemType};
use crate::challenges::ChallengeTrait;
use crate::simulation_settings::{Challenge, SimSettings};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::{color_from_team, FromTeam};
use crate::{Point, MAP_SIZE, PROJECTILE_LIFETIME, SHOOT_SPEED};

#[derive(Default, Clone)]
pub struct BrainLegionCounted {
	counts: HashMap<Entity, u32>,
}

impl BrainLegionTrait for BrainLegionCounted {
	fn add_entity(&mut self, world: &mut World, position: Point, velocity: Option<Point>, blue: f32) {
		let mut found_ent = None;

		// Try and find if an identical entity already exists.
		let mut query = <(&PositionComp, &VelocityComp, &ColorComp)>::query();
		for chunk in query.iter_chunks(world) {
			chunk.into_iter_entities().for_each(|(ent, (pos, vel, col))| {
				if pos.pos == position && vel.vel == velocity.unwrap() && col.blue == blue {
					found_ent = Some(ent);
				}
			});
		}

		if let Some(found_ent) = found_ent {
			let count = self.counts.get_mut(&found_ent).unwrap();
			*count += 1;
			return;
		}
		// Try and find if an identical entity already exists.
		let ent_id = if let Some(velocity) = velocity {
			world.push((
				PositionComp { pos: position },
				VelocityComp { vel: velocity },
				ColorComp { blue },
				UniverseComp { universe_id: 0 },
			))
		} else {
			world.push((PositionComp { pos: position }, ColorComp { blue }, UniverseComp {
				universe_id: 0,
			}))
		};
		let found = self.counts.insert(ent_id, 1);
		assert_eq!(found, None);
	}

	fn add_entity_blob(&mut self, world: &mut World, position: Point, blob: Vec<u8>, team: Option<usize>) {
		let mut found_ent = None;

		// Try and find if an identical entity already exists.
		let mut query = <(&PositionComp, &Vec<u8>)>::query();
		for chunk in query.iter_chunks(world) {
			chunk.into_iter_entities().for_each(|(ent, (pos, blob))| {
				if pos.pos == position && blob == blob {
					found_ent = Some(ent);
				}
			});
		}

		if let Some(found_ent) = found_ent {
			let count = self.counts.get_mut(&found_ent).unwrap();
			*count += 1;
			return;
		}
		let ent_id = if let Some(team) = team {
			world.push((
				PositionComp { pos: position },
				BlobComp { blob },
				UniverseComp { universe_id: 0 },
				ColorComp {
					blue: color_from_team(team),
				},
				TeamComp { team },
			))
		} else {
			world.push((
				PositionComp { pos: position },
				blob,
				UniverseComp { universe_id: 0 },
				ColorComp { blue: 1.0 },
			))
		};

		let found = self.counts.insert(ent_id, 1);
		assert_eq!(found, None);
	}
}
#[derive(Default, Clone)]
pub struct BrainLegionDupey {}

pub trait BrainLegionTrait: Clone {
	fn add_entity(&mut self, world: &mut World, position: Point, velocity: Option<Point>, blue: f32);
	fn add_entity_blob(&mut self, world: &mut World, position: Point, blob: Vec<u8>, team: Option<usize>);
}

impl BrainLegionTrait for BrainLegionDupey {
	fn add_entity(&mut self, world: &mut World, position: Point, velocity: Option<Point>, blue: f32) {
		if let Some(velocity) = velocity {
			world.push((
				PositionComp { pos: position },
				VelocityComp { vel: velocity },
				ColorComp { blue },
				UniverseComp { universe_id: 0 },
			));
		} else {
			world.push((PositionComp { pos: position }, ColorComp { blue }, UniverseComp {
				universe_id: 0,
			}));
		}
	}

	fn add_entity_blob(&mut self, world: &mut World, position: Point, blob: Vec<u8>, team: Option<usize>) {
		if let Some(team) = team {
			world.push((
				PositionComp { pos: position },
				BlobComp { blob },
				UniverseComp { universe_id: 0 },
				ColorComp {
					blue: color_from_team(team),
				},
				TeamComp { team },
			))
		} else {
			world.push((
				PositionComp { pos: position },
				BlobComp { blob },
				UniverseComp { universe_id: 0 },
				ColorComp { blue: 1.0 },
			))
		};
	}
}

pub struct BrainLegion<T: BrainLegionTrait + Clone> {
	schedule:   Option<Schedule>,
	world:      World,
	trait_data: T,
}

pub fn make_unit(world: &mut World, pos: Vec2, vel: Vec2, team: usize, universe_id: usize) {
	let command_buffer = &mut CommandBuffer::new(world);
	let ent = command_buffer.push((
		PositionComp { pos },
		VelocityComp { vel },
		TeamComp { team },
		ColorComp {
			blue: color_from_team(team),
		},
		UniverseComp { universe_id },
		ShooterComp { cooldown: 0.0 },
	));
	command_buffer.flush(world, &mut Resources::default());
}

fn make_projectile(buffer: &mut CommandBuffer, pos: Vec2, target: Vec2, universe_id: usize) {
	let vel = (target - pos).normalize() * 100.0;
	buffer.push((
		PositionComp { pos },
		VelocityComp { vel },
		ColorComp { blue: 0.8 },
		TimedLifeComp {
			time_left: PROJECTILE_LIFETIME,
		},
		UniverseComp { universe_id },
	));
}

#[system(for_each)]
fn velocity(#[resource] dt: &f32, pos: &mut PositionComp, vel: &VelocityComp) {
	pos.pos += vel.vel * *dt;
}

#[system(for_each)]
fn acceleration(#[resource] dt: &f32, vel: &mut VelocityComp, acc: &AccelerationComp) {
	vel.vel += acc.acc * *dt;
}

#[system(for_each)]
fn map_edge(pos: &mut PositionComp) {
	pos.pos.x = pos.pos.x.rem_euclid(MAP_SIZE);
	pos.pos.y = pos.pos.y.rem_euclid(MAP_SIZE);
}

// Decrement the time left on all entities with a TimedLife component
#[system(for_each)]
fn update_timed_life(#[resource] dt: &f32, time: &mut TimedLifeComp) {
	time.time_left -= *dt;
}

// Shoot projectiles at the nearest enemy
#[system(for_each)]
fn shoot(
	#[resource] dt: &f32,
	#[resource] other_entities: &Vec<(PositionComp, TeamComp, UniverseComp)>,
	#[resource] settings: &SimSettings,
	pos: &PositionComp,
	team: &TeamComp,
	shooter: &mut ShooterComp,
	universe: &UniverseComp,
	buffer: &mut CommandBuffer,
) {
	shooter.cooldown -= *dt;
	if shooter.cooldown <= 0.0 {
		let mut closest_dist = f32::MAX;
		let mut closest_pos = Vec2::ZERO;
		for (other_pos, other_team, other_universe) in other_entities.iter() {
			if other_team.team == team.team || other_universe.universe_id != universe.universe_id {
				continue;
			}
			let dist = (pos.pos - other_pos.pos).length();
			if dist < closest_dist {
				closest_dist = dist;
				closest_pos = other_pos.pos;
			}
		}
		// let a = if let Challenge::Rts { .. } = &settings.simulation_settings.challenge_type {
		//     ..
		// }else{
		//     panic!("");
		// };
		// if a == 2.0{
		//
		// }
		if closest_dist < settings.rts_range {
			make_projectile(buffer, pos.pos, closest_pos, universe.universe_id);
			shooter.cooldown = SHOOT_SPEED;
		}
	}
}

#[system(for_each)]
fn edit_team_two_image(blob: &mut BlobComp, team: &TeamComp) {
	if team.team == 1 {
		crate::challenges::ch_image_editing::edit_image(&mut blob.blob);
	}
}

#[system(for_each)]
fn paint_nearest(
	#[resource] pos_color: &Vec<(PositionComp, ColorComp)>,
	#[resource] settings: &SimSettings,
	pos: &PositionComp,
	color: &mut ColorComp,
) {
	let mut closest_dist = f32::MAX;
	let mut closest_color = &ColorComp { blue: 0.0 };
	for (other_pos, other_color) in pos_color.iter() {
		let dist = (pos.pos - other_pos.pos).length();
		if dist < closest_dist && dist > 0.0 {
			closest_dist = dist;
			closest_color = other_color;
		}
	}
	color.blend(closest_color, &settings);
}

// Delete entities that have expired
#[system(for_each)]
fn delete_expired(time: &TimedLifeComp, entity: &Entity, command_buffer: &mut CommandBuffer) {
	if time.time_left <= 0.0 {
		command_buffer.remove(*entity);
	}
}
#[system(for_each)]
fn edit_team_one_color(team: &TeamComp, color: &mut ColorComp) {
	if team.team == 1 {
		color.blue = 0.5;
	}
}

impl<T: Default + BrainLegionTrait> BrainLegion<T> {
	pub fn new() -> Self {
		let mut world = World::default();
		Self {
			world,
			schedule: None,

			trait_data: T::default(),
		}
	}
}

impl<T: BrainLegionTrait + 'static> Brain for BrainLegion<T> {
	fn add_entity_unit(&mut self, position: Point, velocity: Point, team: usize, universe_id: usize) {
		make_unit(&mut self.world, position, velocity, team, universe_id);
	}

	fn add_entity(&mut self, position: Point, velocity: Option<Point>, blue: f32) {
		self.trait_data.add_entity(&mut self.world, position, velocity, blue);
	}

	fn add_entity_blob(&mut self, position: Point, blob: &BlobComp, blue: f32, team: Option<usize>) {
		self.trait_data
			.add_entity_blob(&mut self.world, position, blob.blob.clone(), team);
	}

	fn get_entities(&mut self, universe_id: usize) -> Vec<ExportEntity> {
		let mut query = <(Read<PositionComp>, Read<ColorComp>, Read<UniverseComp>)>::query();
		let mut entities = Vec::new();

		for chunk in query.iter_chunks(&self.world) {
			chunk.into_iter_entities().for_each(|(ent, (pos, color, universe))| {
				if universe.universe_id == universe_id {
					let ent_u64 = unsafe { std::mem::transmute(ent) };
					entities.push(ExportEntity {
						position:  pos.pos,
						blue:      color.blue,
						entity_id: ent_u64,
					});
				}
			});
		}
		entities
	}

	fn get_image<'a>(&'a mut self, entity_id: u64) -> Cow<Vec<u8>> {
		let entity = unsafe { std::mem::transmute::<u64, Entity>(entity_id) };

		let entry_ref: EntryRef<'a> = self.world.entry_ref(entity).unwrap();

		let blob = &entry_ref.get_component::<BlobComp>().as_ref().unwrap().blob;

		// let blob_as_pointer: *const Vec<u8> = blob;
		//
		// let blob_as_ref_unsafe = unsafe { &*blob_as_pointer };
		//
		// Cow::Borrowed(blob_as_ref_unsafe)
		let cloned = blob.clone();
		Cow::Owned(cloned)
	}

	fn init(&mut self, systems: &Vec<SystemType>) {
		let mut schedule = Schedule::builder();
		for system in systems.iter() {
			match system {
				SystemType::EditTeamOneColor => schedule.add_system(edit_team_one_color_system()),
				SystemType::Velocity => schedule.add_system(velocity_system()),
				SystemType::Acceleration => schedule.add_system(acceleration_system()),
				SystemType::MapEdge => schedule.add_system(map_edge_system()),
				SystemType::UpdateTimedLife => schedule.add_system(update_timed_life_system()),
				SystemType::Shoot => schedule.add_system(shoot_system()),
				SystemType::DeleteExpired => schedule.add_system(delete_expired_system()),
				SystemType::PaintNearest => schedule.add_system(paint_nearest_system()),
				SystemType::EditTeamOneImage => schedule.add_system(edit_team_two_image_system()),
			};
		}
		self.schedule = Some(schedule.build());
	}

	fn tick_systems(&mut self, delta: f32, settings: &SimSettings, systems: &Vec<SystemType>) {
		let mut resources = Resources::default();
		resources.insert(delta);
		resources.insert(settings.clone());

		let mut pos_team_universe = Vec::new();
		let mut query = <(&PositionComp, &TeamComp, &UniverseComp)>::query();
		for (pos, team, universe) in query.iter(&self.world) {
			pos_team_universe.push((*pos, *team, *universe));
		}

		let mut pos_color = Vec::new();
		let mut query = <(&PositionComp, &ColorComp)>::query();
		for (pos, color) in query.iter(&self.world) {
			pos_color.push((*pos, *color));
		}

		resources.insert(pos_team_universe);
		resources.insert(pos_color);
		resources.insert(settings.clone());

		let schedule = self.schedule.as_mut().unwrap();
		schedule.execute(&mut self.world, &mut resources);
	}

	fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &SimSettings) {
		match system {
			SystemType::Velocity => {
				let mut query = <(&mut PositionComp, &VelocityComp)>::query();
				for (mut pos, vel) in query.iter_mut(&mut self.world) {
					velocity(&delta, pos, vel)
				}
			}
			SystemType::Acceleration => {
				let mut query = <(&mut VelocityComp, &AccelerationComp)>::query();
				for (mut vel, acc) in query.iter_mut(&mut self.world) {
					acceleration(&delta, vel, acc)
				}
			}
			SystemType::MapEdge => {
				let mut query = <(&mut PositionComp)>::query();
				for (mut pos) in query.iter_mut(&mut self.world) {
					map_edge(pos);
				}
			}
			SystemType::UpdateTimedLife => {
				let mut query = <(&mut TimedLifeComp)>::query();
				for (mut time) in query.iter_mut(&mut self.world) {
					update_timed_life(&delta, time);
				}
			}
			SystemType::Shoot => {
				let mut pos_team_universe = Vec::new();
				let mut query = <(&PositionComp, &TeamComp, &UniverseComp)>::query();
				for (pos, team, universe) in query.iter(&self.world) {
					pos_team_universe.push((*pos, *team, *universe));
				}
				let mut buffer = CommandBuffer::new(&mut self.world);

				let mut query = <(&PositionComp, &TeamComp, &UniverseComp, &mut ShooterComp)>::query();
				for (pos, team, universe, mut shooter) in query.iter_mut(&mut self.world) {
					shoot(&delta, &pos_team_universe, settings, pos, team, shooter, universe, &mut buffer);
				}
				buffer.flush(&mut self.world, &mut Resources::default());
			}
			SystemType::DeleteExpired => {
				let mut buffer = CommandBuffer::new(&self.world);
				let mut query = <(&TimedLifeComp)>::query();

				for chunk in query.iter_chunks(&self.world) {
					chunk.into_iter_entities().for_each(|(ent, time)| {
						delete_expired(time, &ent, &mut buffer);
					});
				}
				buffer.flush(&mut self.world, &mut Resources::default());
			}
			SystemType::PaintNearest => {
				let mut pos_color = Vec::new();
				let mut query = <(&PositionComp, &ColorComp)>::query();
				for (pos, color) in query.iter(&self.world) {
					pos_color.push((*pos, *color));
				}
				let mut query = <(Entity, &PositionComp, &mut ColorComp)>::query();
				for (entity, pos, color) in query.iter_mut(&mut self.world) {
					paint_nearest(&pos_color, settings, pos, color);
				}
			}
			SystemType::EditTeamOneImage => {
				unimplemented!("Single thread not supported.")
			}
			SystemType::EditTeamOneColor => {
				unimplemented!("Single thread not supported.")
			}
		}
	}

	fn get_name(&self) -> String {
		String::from("Legion scheduled")
	}

	fn clone_box(&self) -> Box<dyn Brain> {
		// create a registry which uses strings as the external type ID
		let mut registry = Registry::<String>::default();
		registry.register::<PositionComp>("position".to_string());
		registry.register::<VelocityComp>("velocity".to_string());
		registry.register::<UniverseComp>("universe".to_string());
		registry.register::<ColorComp>("color".to_string());

		let filter = legion::any();
		let entity_serializer = Canon::default();
		let json = serde_json::to_value(&self.world.as_serializable(filter, &registry, &entity_serializer))
			.expect("Failed to serialize world!");

		// registries are also serde deserializers
		use serde::de::DeserializeSeed;
		let new_world: World = registry
			.as_deserialize(&entity_serializer)
			.deserialize(json)
			.expect("Failed to deserialize world!");

		return Box::new(Self {
			schedule:   None,
			world:      new_world,
			trait_data: self.trait_data.clone(),
		});

		// let bytes = bincode::serialize(&self.world.as_serializable(filter, &registry, &entity_serializer))
		// 	.expect("Failed to serialize world!");
		//
		// let d = bincode::de::Deserializer::new(&mut some_reader, SizeLimit::new());
		// // serde::Deserialize::deserialize(&mut deserializer);
		// // let bytes_read = d.bytes_read();
		// // let e: World = bincode::deserialize(&bytes).unwrap();
		// registry.as_deserialize(&entity_serializer).deserialize(&d).unwrap();

		// // registries are also serde deserializers
		// use serde::de::DeserializeSeed;
		// let _world: World = registry
		// 	.as_deserialize(&entity_serializer)
		// 	.deserialize(json)
		// 	.expect("Failed to deserialize world!");
	}
}
