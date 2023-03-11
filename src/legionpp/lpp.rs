use std::any::TypeId;
use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use ggez::filesystem::create;
use legion::Entity;
use trait_bound_typemap::{AnyTypeMap, CloneTypeMap, TypeMap, TypeMapKey};

use crate::legionpp::cupboard::{Cupboard, Shelf, ShelfRef};
use crate::utils::HashMe;

pub type TypeSig = BTreeSet<TypeId>;

pub struct Lpp {
	pub cupboards:       CloneTypeMap,
	pub lentities:       HashMap<Lentity, InternalEntity>,
	pub archetypes:      HashMap<TypeSig, Vec<Lentity>>,
	pub uniques_by_type: HashMap<BTreeSet<(TypeId, ShelfRef)>, Vec<Lentity>>,
}

pub struct InternalEntity {
	pub shelves: HashMap<TypeId, ShelfRef>,
}
pub type Lentity = usize;
pub type GroupedLentity = Lentity;

fn lentity_to_grouped_lentity(lentity: Lentity) -> GroupedLentity {
	assert!(lentity < u32::MAX as usize / 2);
	lentity + u32::MAX as usize / 2
}
fn grouped_lentity_to_lentity(lentity: GroupedLentity) -> Lentity {
	assert!(lentity >= u32::MAX as usize / 2);
	lentity - u32::MAX as usize / 2
}
fn is_grouped_lentity(lentity: usize) -> bool {
	lentity >= u32::MAX as usize / 2
}
fn is_lentity(lentity: usize) -> bool {
	lentity < u32::MAX as usize / 2
}

impl<T: 'static + Clone> TypeMapKey for OurKey<T> {
	type Value = T;
}
pub struct OurKey<T> {
	_t: T,
}
impl Lpp {
	pub fn add<T: Clone + 'static>(&mut self, cupboard: Cupboard<T>) {
		self.cupboards.insert::<OurKey<Cupboard<T>>>(cupboard);
	}

	pub fn new() -> Lpp {
		Lpp {
			cupboards:       CloneTypeMap::new(),
			lentities:       Default::default(),
			archetypes:      Default::default(),
			uniques_by_type: Default::default(),
		}
	}

	pub fn create_entity(&mut self) -> Lentity {
		let lentity = self.lentities.len();
		self.lentities.insert(lentity, InternalEntity {
			shelves: Default::default(),
		});
		return lentity;
	}

	fn create_cupboard_if_needed<T: Clone + Hash + Debug + 'static>(&mut self) {
		if self.cupboards.get::<OurKey<Cupboard<T>>>().is_none() {
			self.add(Cupboard::<T>::new());
		}
	}

	pub fn get_entity(&mut self, lentity: Lentity) -> &mut InternalEntity {
		self.lentities.get_mut(&lentity).expect("Ent doesn't exist")
	}

	pub fn add_component<T: Clone + Hash + Debug + 'static>(&mut self, lentity: Lentity, component: T) {
		self.create_cupboard_if_needed::<T>();
		let mut cupboard = self.cupboards.get_mut::<OurKey<Cupboard<T>>>().unwrap();
		let shelf_ref = cupboard.add_component(component);
		self.get_entity(lentity).shelves.insert(TypeId::of::<T>(), shelf_ref);
	}

	pub fn complete_entity(&mut self, lentity: Lentity) {
		let type_sig = self.get_entity(lentity).shelves.keys().cloned().collect();
		self.archetypes.entry(type_sig).or_insert_with(|| Vec::new()).push(lentity);
	}

	pub fn query_uniques(&mut self, type_sig: Vec<TypeId>) -> Vec<GroupedLentity> {
		self.uniques_by_type.clear();

		let query_results = self.query(type_sig);
		for lentity in query_results {
			let ent_internal = self.get_entity(lentity);
			let mut uniques = BTreeSet::new();
			for (type_id, shelf_ref) in ent_internal.shelves.iter() {
				uniques.insert((*type_id, *shelf_ref));
			}
			self.uniques_by_type.entry(uniques).or_insert_with(|| Vec::new()).push(lentity);
		}
		self.uniques_by_type
			.values()
			.map(|v| lentity_to_grouped_lentity(v[0]))
			.collect()
	}

	pub fn query(&mut self, type_sig: Vec<TypeId>) -> Vec<Lentity> {
		let type_sig: TypeSig = type_sig.into_iter().collect();

		let mut lentities = vec![];

		for archetype in self.archetypes.keys() {
			if type_sig.is_subset(archetype) {
				lentities.extend(self.archetypes.get(archetype).unwrap());
			}
		}

		lentities
	}

	pub fn get_component_ref<T: Clone + Hash + Debug + 'static>(&self, lentity: Lentity) -> Option<&T> {
		let cupboard = self.cupboards.get::<OurKey<Cupboard<T>>>()?;
		let shelf_ref = self.lentities.get(&lentity)?.shelves.get(&TypeId::of::<T>())?;
		let shelf = cupboard.get_shelf(shelf_ref);
		let to_ret = match shelf {
			Shelf::One { data } => Some(data.as_ref().expect("Was it already on loan?")),
			Shelf::Many {
				data_backup: data, ..
			} => Some(&*data),
		};

		to_ret
	}

	pub fn get_component<T: Clone + Hash + Debug + 'static>(&mut self, lentity: Lentity) -> Option<T> {
		let cupboard = self.cupboards.get_mut::<OurKey<Cupboard<T>>>()?;
		let shelf_ref = self.lentities.get_mut(&lentity)?.shelves.get(&TypeId::of::<T>())?;
		let shelf = cupboard.get_shelf_mut(shelf_ref);

		let to_ret = match shelf {
			Shelf::One { data } => {
				assert!(data.is_some(), "Was it already on loan?");
				data.take()
			}
			Shelf::Many {
				data_backup,
				data,
				qty,
			} => Some(*data.take().expect("Was it already on loan?")),
		};

		to_ret
	}

	pub fn return_component<T: Clone + Hash + Debug + 'static>(&mut self, lentity: Lentity, component: T) {
		let cupboard = self.cupboards.get_mut::<OurKey<Cupboard<T>>>().unwrap();
		let internal_ent = self.lentities.get_mut(&lentity).expect("Ent doesn't exist");
		let shelf_ref = internal_ent.shelves.get(&TypeId::of::<T>()).unwrap();
		let shelf = cupboard.get_shelf_mut(shelf_ref);

		let maybe_shelf_new_comp = match shelf {
			Shelf::One { .. } => None,
			Shelf::Many { .. } => cupboard.hash_shelf_lookup.get(&component.hash_me()).cloned(),
		};

		let cupboard = self.cupboards.get_mut::<OurKey<Cupboard<T>>>().unwrap();
		let internal_ent = self.lentities.get_mut(&lentity).expect("Ent doesn't exist");
		let shelf_ref = internal_ent.shelves.get_mut(&TypeId::of::<T>()).unwrap();
		let shelf = cupboard.get_shelf_mut(shelf_ref);

		println!("Returning component");

		match shelf {
			Shelf::One { data } => {
				*data = Some(component);
			}
			Shelf::Many {
				data_backup,
				data,
				qty,
			} => {
				let mut identical = false;

				if let Some(maybe_shelf_new_comp) = maybe_shelf_new_comp {
					if maybe_shelf_new_comp == *shelf_ref {
						identical = true;
					}
				}

				if identical {
					println!("(Unchanged)");
					*data = Some(Box::new(component));
				} else {
					println!("It's changed!!");

					// Decrease the qty of the shelf
					*qty -= 1;

					// if the quantity is 1, set the shelf to be a one
					if *qty == 1 {
						println!("qty == 1");

						let mut new_shelf = Shelf::One {
							data: Some(data_backup.clone()),
						};
						std::mem::swap(shelf, &mut new_shelf);
					} else {
						println!("qty != 1");
						*data = Some(Box::new(data_backup.clone()));
					}

					let new_ent_shelf_ref = cupboard.add_component(component);
					*shelf_ref = new_ent_shelf_ref;
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use plotters::style::text_anchor::Pos;

	use super::*;
	use crate::brains::com::*;
	use crate::Point;

	#[test]
	fn basic() {
		let mut lpp = Lpp::new();
		let position_comp = PositionComp {
			pos: Point::new(1.0, 0.0),
		};
		let mut entity = lpp.create_entity();
		lpp.add_component(entity, position_comp);

		let velocity_comp = VelocityComp {
			vel: Point::new(0.0, 333.0),
		};
		lpp.add_component(entity, velocity_comp);

		lpp.complete_entity(entity);

		// Query for all entities with a position component
		let mut matching_entities =
			lpp.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

		assert_eq!(matching_entities.len(), 1);
		for entity in &matching_entities {
			let mut position = lpp.get_component::<PositionComp>(*entity).unwrap();
			let velocity = lpp.get_component_ref::<VelocityComp>(*entity).unwrap();
			// Increment the position by the velocity
			position.pos += velocity.vel;
			println!("Entity {:?} has position {:?} and velocity {:?}", entity, position, velocity);
			lpp.return_component(*entity, position);
		}
		// Assert that the position has been incremented
		let position = lpp.get_component::<PositionComp>(entity).unwrap();
		assert_eq!(position.pos, Point::new(1.0, 333.0));
	}
	#[test]
	fn test_position_deduplication() {
		let mut lpp = Lpp::new();
		for i in 0..2 {
			let mut entity = lpp.create_entity();
			lpp.add_component(entity, PositionComp {
				pos: Point::new(0.0, 0.0),
			});
			lpp.add_component(entity, VelocityComp {
				vel: Point::new(0.0, i as f32),
			});
			lpp.complete_entity(entity);
		}
		// Query for all entities with a position component
		let mut matching_entities =
			lpp.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

		assert_eq!(matching_entities.len(), 2);
		for entity in &matching_entities {
			let mut position = lpp.get_component::<PositionComp>(*entity).unwrap();
			let velocity = lpp.get_component_ref::<VelocityComp>(*entity).unwrap();
			// Increment the position by the velocity
			position.pos += velocity.vel;
			lpp.return_component(*entity, position);
		}
		// Assert that both entities have correct positions
		let mut matching_entities =
			lpp.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);
		let mut expected_positions = vec![Point::new(0.0, 0.0), Point::new(0.0, 1.0)];
		for entity in &matching_entities {
			let position = lpp.get_component_ref::<PositionComp>(*entity).unwrap();
			// Remove the position from the expected positions

			println!("{}", &position.pos);
			let index = expected_positions
				.iter()
				.position(|x| *x == position.pos)
				.expect(format!("Position was wrong! {}", position.pos).as_str());
			expected_positions.remove(index);
		}
		assert_eq!(expected_positions.len(), 0);
	}
	#[test]
	pub fn test_vel_pos_acc() {
		let mut lpp = Lpp::new();
		for i in 0..10 {
			let mut entity = lpp.create_entity();
			lpp.add_component(entity, PositionComp {
				pos: Point::new(2.0, 2.0),
			});
			lpp.add_component(entity, VelocityComp {
				vel: Point::new(0.0, 2.0),
			});
			lpp.complete_entity(entity);
		}
		for i in 0..5 {
			let mut entity = lpp.create_entity();
			lpp.add_component(entity, PositionComp {
				pos: Point::new(2.0, 0.0),
			});
			lpp.add_component(entity, VelocityComp {
				vel: Point::new(0.0, 4.0),
			});
			lpp.add_component(entity, AccelerationComp {
				acc: Point::new(0.0, 2.0),
			});
			lpp.complete_entity(entity);
		}
		for i in 0..2 {
			let mut entity = lpp.create_entity();
			lpp.add_component(entity, PositionComp {
				pos: Point::new(2.0, 10.0),
			});
			lpp.complete_entity(entity);
		}
		{
			// Velocity

			// Query for all entities with a position component
			let mut matching_entities =
				lpp.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

			assert_eq!(matching_entities.len(), 15);
			for entity in &matching_entities {
				let mut position = lpp.get_component::<PositionComp>(*entity).unwrap();
				let velocity = lpp.get_component_ref::<VelocityComp>(*entity).unwrap();
				// Increment the position by the velocity
				position.pos += velocity.vel;
				lpp.return_component(*entity, position);
			}
		}
		{
			// Acc

			let mut matching_entities = lpp.query(vec![
				TypeId::of::<PositionComp>(),
				TypeId::of::<VelocityComp>(),
				TypeId::of::<AccelerationComp>(),
			]);
			assert_eq!(matching_entities.len(), 5);
			for entity in &matching_entities {
				let mut vel = lpp.get_component::<VelocityComp>(*entity).unwrap();
				let acc = lpp.get_component_ref::<AccelerationComp>(*entity).unwrap();
				// Increment the position by the velocity
				vel.vel += acc.acc;
				lpp.return_component(*entity, vel);
			}
		}
		{
			// Velocity

			let mut matching_entities =
				lpp.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

			assert_eq!(matching_entities.len(), 15);
			for entity in &matching_entities {
				let mut position = lpp.get_component::<PositionComp>(*entity).unwrap();
				let velocity = lpp.get_component_ref::<VelocityComp>(*entity).unwrap();
				// Increment the position by the velocity
				position.pos += velocity.vel;
				lpp.return_component(*entity, position);
			}
		}

		// Assert that both entities have correct positions

		let mut expected_positions = vec![];

		for i in 0..10 {
			expected_positions.push(Point::new(2.0, 6.0));
		}
		for i in 0..7 {
			expected_positions.push(Point::new(2.0, 10.0));
		}
		let mut matching_entities = lpp.query(vec![TypeId::of::<PositionComp>()]);
		for entity in &matching_entities {
			let position = lpp.get_component_ref::<PositionComp>(*entity).unwrap();
			let index = expected_positions
				.iter()
				.position(|x| *x == position.pos)
				.expect(format!("Position was wrong! {}", position.pos).as_str());
			expected_positions.remove(index);
		}
		assert_eq!(expected_positions.len(), 0);

		let mut expected_velocities = vec![];
		for i in 0..10 {
			expected_velocities.push(Point::new(0.0, 2.0));
		}
		for i in 0..5 {
			expected_velocities.push(Point::new(0.0, 6.0));
		}
		let mut matching_entities = lpp.query(vec![TypeId::of::<VelocityComp>()]);
		for entity in &matching_entities {
			let velocity = lpp.get_component_ref::<VelocityComp>(*entity).unwrap();
			let index = expected_velocities
				.iter()
				.position(|x| *x == velocity.vel)
				.expect(format!("Vel was wrong! {}", velocity.vel).as_str());
			expected_velocities.remove(index);
		}
		assert_eq!(expected_velocities.len(), 0);
	}
}