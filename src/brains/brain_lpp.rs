use std::any::TypeId;

use legion::systems::CommandBuffer;
use legion::{Entity, IntoQuery, Resources, Schedule, World};

use crate::brains::brain_legion::BrainLegionTrait;
use crate::brains::com::{
	AccelerationComp,
	BlobComp,
	ColorComp,
	ExportEntity,
	PositionComp,
	ShooterComp,
	TeamComp,
	TimedLifeComp,
	UniverseComp,
	VelocityComp,
};
use crate::brains::{Brain, SystemType};
use crate::legionpp::lpp::Lpp;
use crate::simulation_settings::SimSettings;
use crate::utils::color_from_team;
use crate::{Point, MAP_SIZE, PROJECTILE_LIFETIME, SHOOT_SPEED};

pub struct BrainLpp {
	world: Lpp,
}

pub fn make_unit(world: &mut World, pos: Vec2, vel: Vec2, team: usize, universe_id: usize) {}

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
		if dist < closest_dist {
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

impl BrainLpp {
	pub fn new() -> Self {
		Self { world: Lpp::new() }
	}
}

impl Brain for BrainLpp {
	fn add_entity_unit(&mut self, position: Point, velocity: Point, team: usize, universe_id: usize) {
		let mut entity = self.world.create_entity();
		self.world.add_component(entity, PositionComp { pos: position });
		self.world.add_component(entity, VelocityComp { vel: velocity });
		self.world.add_component(entity, UniverseComp { universe_id });
		self.world.add_component(entity, ShooterComp { cooldown: 0.0 });
		self.world.add_component(entity, ColorComp {
			blue: color_from_team(team),
		});
		self.world.complete_entity(entity);
	}

	fn add_entity(&mut self, position: Point, velocity: Option<Point>, blue: f32) {
		let mut entity = self.world.create_entity();
		self.world.add_component(entity, PositionComp { pos: position });
		if let Some(velocity) = velocity {
			self.world.add_component(entity, VelocityComp { vel: velocity });
		}
		self.world.add_component(entity, UniverseComp { universe_id: 0 });
		self.world.add_component(entity, ColorComp { blue });
		self.world.complete_entity(entity);
	}

	fn add_entity_blob(&mut self, position: Point, blob: Vec<u8>, blue: f32) {
		let mut entity = self.world.create_entity();
		self.world.add_component(entity, PositionComp { pos: position });
		self.world.add_component(entity, ColorComp { blue });
		self.world.add_component(entity, BlobComp { blob });
		self.world.complete_entity(entity);
	}

	fn get_entities(&mut self, universe_id: usize) -> Vec<ExportEntity> {
		let mut matching_entities = self.world.query(vec![
			TypeId::of::<PositionComp>(),
			TypeId::of::<ColorComp>(),
			TypeId::of::<UniverseComp>(),
		]);
		let mut entities = Vec::new();
		for entity in &matching_entities {
			if self.world.get_component_ref::<UniverseComp>(*entity).unwrap().universe_id == universe_id {
				entities.push(ExportEntity {
					position: self.world.get_component_ref::<PositionComp>(*entity).unwrap().pos,
					blue:     self.world.get_component_ref::<ColorComp>(*entity).unwrap().blue,
				});
			}
		}
		entities
	}

	fn init(&mut self, systems: &Vec<SystemType>) {}

	fn tick_systems(&mut self, delta: f32, settings: &SimSettings, systems: &Vec<SystemType>) {
		for system in systems {
			self.tick_system(system, delta, settings);
		}
	}

	fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &SimSettings) {
		match system {
			SystemType::Velocity => {
				// let mut query = <(&mut PositionComp, &VelocityComp)>::query();
				// for (mut pos, vel) in query.iter_mut(&mut self.world) {
				// 	velocity(&delta, pos, vel)
				// }
				let mut matching_entities = self
					.world
					.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

				for entity in &matching_entities {
					let mut position = self.world.get_component::<PositionComp>(*entity).unwrap();
					let velocity = self.world.get_component_ref::<VelocityComp>(*entity).unwrap();

					position.pos += velocity.vel;
					self.world.return_component(*entity, position);
				}
			}
			SystemType::Acceleration => {
				// let mut query = <(&mut VelocityComp, &AccelerationComp)>::query();
				// for (mut vel, acc) in query.iter_mut(&mut self.world) {
				// 	acceleration(&delta, vel, acc)
				// }
				let mut matching_entities = self.world.query(vec![
					TypeId::of::<VelocityComp>(),
					TypeId::of::<AccelerationComp>(),
				]);

				for entity in &matching_entities {
					let mut velocity = self.world.get_component::<VelocityComp>(*entity).unwrap();
					let acceleration = self.world.get_component_ref::<AccelerationComp>(*entity).unwrap();

					velocity.vel += acceleration.acc;
					self.world.return_component(*entity, velocity);
				}
			}
			SystemType::MapEdge => {
				// 	let mut query = <(&mut PositionComp)>::query();
				// 	for (mut pos) in query.iter_mut(&mut self.world) {
				// 		map_edge(pos);
				// 	}

				let mut matching_entities = self.world.query(vec![TypeId::of::<PositionComp>()]);

				for entity in &matching_entities {
					let mut position = self.world.get_component::<PositionComp>(*entity).unwrap();
					position.pos.x = position.pos.x.rem_euclid(MAP_SIZE);
					position.pos.y = position.pos.y.rem_euclid(MAP_SIZE);

					self.world.return_component(*entity, position);
				}
			}
			SystemType::UpdateTimedLife => {
				// let mut query = <(&mut TimedLifeComp)>::query();
				// for (mut time) in query.iter_mut(&mut self.world) {
				// 	update_timed_life(&delta, time);
				// }
			}
			SystemType::Shoot => {
				// let mut pos_team_universe = Vec::new();
				// let mut query = <(&PositionComp, &TeamComp, &UniverseComp)>::query();
				// for (pos, team, universe) in query.iter(&self.world) {
				// 	pos_team_universe.push((*pos, *team, *universe));
				// }
				// let mut buffer = CommandBuffer::new(&mut self.world);
				//
				// let mut query = <(&PositionComp, &TeamComp, &UniverseComp, &mut ShooterComp)>::query();
				// for (pos, team, universe, mut shooter) in query.iter_mut(&mut self.world) {
				// 	shoot(&delta, &pos_team_universe, settings, pos, team, shooter, universe, &mut buffer);
				// }
				// buffer.flush(&mut self.world, &mut Resources::default());
			}
			SystemType::DeleteExpired => {
				// let mut buffer = CommandBuffer::new(&self.world);
				// let mut query = <(&TimedLifeComp)>::query();
				//
				// for chunk in query.iter_chunks(&self.world) {
				// 	chunk.into_iter_entities().for_each(|(ent, time)| {
				// 		delete_expired(time, &ent, &mut buffer);
				// 	});
				// }
				// buffer.flush(&mut self.world, &mut Resources::default());
			}
			SystemType::PaintNearest => {
				// let mut pos_color = Vec::new();
				// let mut query = <(&PositionComp, &ColorComp)>::query();
				// for (pos, color) in query.iter(&self.world) {
				// 	pos_color.push((*pos, *color));
				// }
				// let mut query = <(Entity, &PositionComp, &mut ColorComp)>::query();
				// for (entity, pos, color) in query.iter_mut(&mut self.world) {
				// 	paint_nearest(&pos_color, settings, pos, color);
				// }
			}
		}
	}

	fn get_name(&self) -> String {
		String::from("Legion scheduled")
	}
}
