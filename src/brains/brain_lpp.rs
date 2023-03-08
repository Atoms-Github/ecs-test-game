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
		self.world.add_component(entity, UniverseComp { universe_id: 0 });
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

					position.pos += velocity.vel * delta;
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

					velocity.vel += acceleration.acc * delta;
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
				let mut pos_color = Vec::new();
				let mut matching_entities =
					self.world.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<ColorComp>()]);

				for entity in &matching_entities {
					let pos = self.world.get_component_ref::<PositionComp>(*entity).unwrap();
					let color = self.world.get_component_ref::<ColorComp>(*entity).unwrap();
					pos_color.push((*pos, *color));
				}

				let mut matching_entities =
					self.world.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<ColorComp>()]);

				for entity in &matching_entities {
					let mut color = self.world.get_component::<ColorComp>(*entity).unwrap();
					let pos = self.world.get_component_ref::<PositionComp>(*entity).unwrap();
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

					self.world.return_component(*entity, color);
				}
			}
		}
	}

	fn get_name(&self) -> String {
		String::from("Legion scheduled")
	}
}
