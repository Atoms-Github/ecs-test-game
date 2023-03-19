use std::any::TypeId;
use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::hint::black_box;

use futures::stream::iter;
use ggez::filesystem::create;
use legion::Entity;
use trait_bound_typemap::{AnyTypeMap, CloneTypeMap, TypeMap, TypeMapKey};

use crate::rc_ecs::cupboard::{Cupboard, Shelf, ShelfRef};
use crate::utils::HashMe;

pub type TypeSig = BTreeSet<TypeId>;

#[derive(Clone)]
pub struct RcEcs {
	pub cupboards: CloneTypeMap,
	pub lentities: HashMap<Lentity, InternalEntity, nohash_hasher::BuildNoHashHasher<usize>>,
	pub archetypes: HashMap<TypeSig, Vec<Lentity>>,
	/// The u64 is a hashed BTreeSet<(TypeId, ShelfRef)>
	pub uniques_by_type: HashMap<u64, Vec<Lentity>, nohash_hasher::BuildNoHashHasher<u64>>,
	pub current_unique_type_ids: Vec<TypeId>,
	pub indexes: HashMap<TypeId, HashMap<u32, HashSet<Lentity>>>,
}

#[derive(Clone)]
pub struct InternalEntity {
	pub shelves: HashMap<TypeId, ShelfRef>,
	/// The u64 is a hashed Vec<TypeId> to a hashed BTreeSet<(TypeId, ShelfRef)>
	pub sigs:    HashMap<u64, u64, nohash_hasher::BuildNoHashHasher<u64>>,
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

fn index_from_component<T>(comp_ref: &T) -> u32 {
	*unsafe { std::mem::transmute::<&T, &u32>(comp_ref) }
}

impl<T: 'static + Clone> TypeMapKey for OurKey<T> {
	type Value = T;
}
pub struct OurKey<T> {
	_t: T,
}
impl RcEcs {
	pub fn add<T: Clone + 'static>(&mut self, cupboard: Cupboard<T>) {
		self.cupboards.insert::<OurKey<Cupboard<T>>>(cupboard);
	}

	pub fn new() -> RcEcs {
		RcEcs {
			cupboards: CloneTypeMap::new(),
			lentities: Default::default(),
			archetypes: Default::default(),
			uniques_by_type: Default::default(),
			current_unique_type_ids: vec![],
			indexes: Default::default(),
		}
	}

	pub fn query_index<T: Copy + 'static>(&self, desired_index: i32) -> Vec<Lentity> {
		self.indexes
			.get(&TypeId::of::<T>())
			.unwrap()
			.get(&(desired_index as u32))
			.unwrap()
			.iter()
			.cloned()
			.collect()
	}

	pub fn create_index<T: Copy + 'static>(&mut self) {
		let type_id = TypeId::of::<T>();
		self.indexes.insert(type_id, Default::default());
	}

	pub fn create_entity(&mut self) -> Lentity {
		let lentity = self.lentities.len();
		self.lentities.insert(lentity, InternalEntity {
			shelves: Default::default(),
			sigs:    Default::default(),
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

		let type_id = TypeId::of::<T>();
		self.get_entity(lentity).shelves.insert(type_id, shelf_ref);

		let comp_ref: &T = self.get_component_ref::<T>(lentity).unwrap();
		let new_index = index_from_component(comp_ref);

		if let Some(index) = self.indexes.get_mut(&type_id) {
			index.entry(new_index).or_insert_with(|| HashSet::new()).insert(lentity);
		}
	}

	fn update_entity_index<T: 'static>(&mut self, lentity: Lentity, new_index: u32, old_index: u32) {
		let type_id = TypeId::of::<T>();
		self.indexes.get_mut(&type_id).map(|index| {
			index.entry(old_index).or_insert_with(|| HashSet::new()).remove(&lentity);
			index.entry(new_index).or_insert_with(|| HashSet::new()).insert(lentity);
		});
	}

	pub fn add_component_ref<T: Clone + Hash + Debug + 'static>(&mut self, lentity: Lentity, component: &T) {
		self.create_cupboard_if_needed::<T>();
		let mut cupboard = self.cupboards.get_mut::<OurKey<Cupboard<T>>>().unwrap();
		let shelf_ref = cupboard.add_component_ref(component);
		self.get_entity(lentity).shelves.insert(TypeId::of::<T>(), shelf_ref);
	}

	pub fn complete_entity(&mut self, lentity: Lentity) {
		let type_sig = self.get_entity(lentity).shelves.keys().cloned().collect();
		self.archetypes.entry(type_sig).or_insert_with(|| Vec::new()).push(lentity);
	}

	pub fn query_uniques(&mut self, type_sig_vec: Vec<TypeId>) -> Vec<GroupedLentity> {
		self.current_unique_type_ids = type_sig_vec.clone();
		self.uniques_by_type.clear();
		let type_sig_set_hashed = type_sig_vec.iter().cloned().collect::<Vec<TypeId>>().hash_me();

		let query_results = self.query(type_sig_vec.clone());
		for lentity in query_results {
			let ent_internal = self.get_entity(lentity); // 1ms
			let mut uniques = *ent_internal.sigs.entry(type_sig_set_hashed).or_insert_with(|| {
				let mut new_uniques = BTreeSet::new();
				for (type_id, shelf_ref) in ent_internal.shelves.iter() {
					if type_sig_vec.contains(type_id) {
						new_uniques.insert((*type_id, *shelf_ref));
					}
				}
				new_uniques.hash_me()
			});

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

	pub fn get_component_ref<T: Clone + Hash + Debug + 'static>(&self, mut lentity: Lentity) -> Option<&T> {
		if is_grouped_lentity(lentity) {
			lentity = grouped_lentity_to_lentity(lentity);
		}
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

	pub fn get_component<T: Clone + Hash + Debug + 'static>(&mut self, mut lentity: Lentity) -> Option<T> {
		if is_grouped_lentity(lentity) {
			lentity = grouped_lentity_to_lentity(lentity);
		}

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
			} => {
				if data.is_none() {
					*data = Some(Box::new(data_backup.clone()));
				}
				Some(*data.take().unwrap())
			}
		};

		to_ret
	}

	pub fn return_component<T: Clone + Hash + Debug + 'static>(
		&mut self,
		mut lentity: Lentity,
		component: T,
	) {
		let is_grouped = is_grouped_lentity(lentity);
		if is_grouped {
			lentity = grouped_lentity_to_lentity(lentity);
		}
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
		let shelf_ref = *internal_ent.shelves.get_mut(&TypeId::of::<T>()).unwrap();
		let shelf = cupboard.get_shelf_mut(&shelf_ref);

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
					if maybe_shelf_new_comp == shelf_ref {
						identical = true;
					}
				}

				if identical {
					*data = Some(Box::new(component));
				} else {
					let new_index = index_from_component(&component);
					let old_index = index_from_component(data_backup);

					// internal_ent.sigs.clear();

					if !is_grouped {
						// Decrease the qty of the shelf
						*qty -= 1;

						// if the quantity is 1, set the shelf to be a one
						if *qty == 1 {
							// println!("qty == 1");

							let mut new_shelf = Shelf::One {
								data: Some(data_backup.clone()),
							};
							std::mem::swap(shelf, &mut new_shelf);
						} else {
							// println!("qty != 1");
						}

						let new_ent_shelf_ref = cupboard.add_component(component);
						*internal_ent.shelves.get_mut(&TypeId::of::<T>()).unwrap() = new_ent_shelf_ref;

						self.update_entity_index::<T>(lentity, new_index, old_index);
					} else {
						// is_grouped. Its a grouped lentity.
						let my_key = self
							.current_unique_type_ids
							.iter()
							.map(|x| (*x, *internal_ent.shelves.get(x).unwrap()))
							.collect::<BTreeSet<(TypeId, ShelfRef)>>()
							.hash_me();
						let duplicates = self.uniques_by_type.get_mut(&my_key).unwrap();

						if *qty == duplicates.len() as u32 {
							*data_backup = component;

							return;
						}
						*qty -= duplicates.len() as u32;

						// if the quantity is 1, set the shelf to be a one
						if *qty == 1 {
							// println!("qty == 1");

							let mut new_shelf = Shelf::One {
								data: Some(data_backup.clone()),
							};
							std::mem::swap(shelf, &mut new_shelf);
						} else if *qty > 1 {
							// println!("qty > 1");
						}

						let new_ent_shelf_ref = cupboard.add_component(component);
						for relocated_ent_id in duplicates.iter() {
							let relocated_ent = self.lentities.get_mut(relocated_ent_id).unwrap();
							*relocated_ent.shelves.get_mut(&TypeId::of::<T>()).unwrap() = new_ent_shelf_ref;
						}
						cupboard.add_qty(new_ent_shelf_ref, duplicates.len() as u32 - 1);

						for lentity in duplicates.clone().iter() {
							self.update_entity_index::<T>(*lentity, new_index, old_index);
						}
					}
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
		let mut rc_ecs = RcEcs::new();
		let position_comp = PositionComp {
			pos: Point::new(1.0, 0.0),
		};
		let mut entity = rc_ecs.create_entity();
		rc_ecs.add_component(entity, position_comp);

		let velocity_comp = VelocityComp {
			vel: Point::new(0.0, 333.0),
		};
		rc_ecs.add_component(entity, velocity_comp);

		rc_ecs.complete_entity(entity);

		// Query for all entities with a position component
		let mut matching_entities =
			rc_ecs.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

		assert_eq!(matching_entities.len(), 1);
		for entity in &matching_entities {
			let mut position = rc_ecs.get_component::<PositionComp>(*entity).unwrap();
			let velocity = rc_ecs.get_component_ref::<VelocityComp>(*entity).unwrap();
			// Increment the position by the velocity
			position.pos += velocity.vel;
			// println!("Entity {:?} has position {:?} and velocity {:?}", entity, position, velocity);
			rc_ecs.return_component(*entity, position);
		}
		// Assert that the position has been incremented
		let position = rc_ecs.get_component::<PositionComp>(entity).unwrap();
		assert_eq!(position.pos, Point::new(1.0, 333.0));
	}
	#[test]
	fn test_position_deduplication() {
		let mut rc_ecs = RcEcs::new();
		for i in 0..2 {
			let mut entity = rc_ecs.create_entity();
			rc_ecs.add_component(entity, PositionComp {
				pos: Point::new(0.0, 0.0),
			});
			rc_ecs.add_component(entity, VelocityComp {
				vel: Point::new(0.0, i as f32),
			});
			rc_ecs.complete_entity(entity);
		}
		// Query for all entities with a position component
		let mut matching_entities =
			rc_ecs.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

		assert_eq!(matching_entities.len(), 2);
		for entity in &matching_entities {
			let mut position = rc_ecs.get_component::<PositionComp>(*entity).unwrap();
			let velocity = rc_ecs.get_component_ref::<VelocityComp>(*entity).unwrap();
			// Increment the position by the velocity
			position.pos += velocity.vel;
			rc_ecs.return_component(*entity, position);
		}
		// Assert that both entities have correct positions
		let mut matching_entities =
			rc_ecs.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);
		let mut expected_positions = vec![Point::new(0.0, 0.0), Point::new(0.0, 1.0)];
		for entity in &matching_entities {
			let position = rc_ecs.get_component_ref::<PositionComp>(*entity).unwrap();
			// Remove the position from the expected positions

			// println!("{}", &position.pos);
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
		let mut rc_ecs = RcEcs::new();
		for i in 0..10 {
			let mut entity = rc_ecs.create_entity();
			rc_ecs.add_component(entity, PositionComp {
				pos: Point::new(2.0, 2.0),
			});
			rc_ecs.add_component(entity, VelocityComp {
				vel: Point::new(0.0, 2.0),
			});
			rc_ecs.complete_entity(entity);
		}
		for i in 0..5 {
			let mut entity = rc_ecs.create_entity();
			rc_ecs.add_component(entity, PositionComp {
				pos: Point::new(2.0, 0.0),
			});
			rc_ecs.add_component(entity, VelocityComp {
				vel: Point::new(0.0, 4.0),
			});
			rc_ecs.add_component(entity, AccelerationComp {
				acc: Point::new(0.0, 2.0),
			});
			rc_ecs.complete_entity(entity);
		}
		for i in 0..2 {
			let mut entity = rc_ecs.create_entity();
			rc_ecs.add_component(entity, PositionComp {
				pos: Point::new(2.0, 10.0),
			});
			rc_ecs.complete_entity(entity);
		}
		{
			// Velocity

			// Query for all entities with a position component
			let mut matching_entities =
				rc_ecs.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

			assert_eq!(matching_entities.len(), 15);
			for entity in &matching_entities {
				let mut position = rc_ecs.get_component::<PositionComp>(*entity).unwrap();
				let velocity = rc_ecs.get_component_ref::<VelocityComp>(*entity).unwrap();
				// Increment the position by the velocity
				position.pos += velocity.vel;
				rc_ecs.return_component(*entity, position);
			}
		}
		{
			// Acc

			let mut matching_entities = rc_ecs.query(vec![
				TypeId::of::<PositionComp>(),
				TypeId::of::<VelocityComp>(),
				TypeId::of::<AccelerationComp>(),
			]);
			assert_eq!(matching_entities.len(), 5);
			for entity in &matching_entities {
				let mut vel = rc_ecs.get_component::<VelocityComp>(*entity).unwrap();
				let acc = rc_ecs.get_component_ref::<AccelerationComp>(*entity).unwrap();
				// Increment the position by the velocity
				vel.vel += acc.acc;
				rc_ecs.return_component(*entity, vel);
			}
		}
		{
			// Velocity

			let mut matching_entities =
				rc_ecs.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

			assert_eq!(matching_entities.len(), 15);
			for entity in &matching_entities {
				let mut position = rc_ecs.get_component::<PositionComp>(*entity).unwrap();
				let velocity = rc_ecs.get_component_ref::<VelocityComp>(*entity).unwrap();
				// Increment the position by the velocity
				position.pos += velocity.vel;
				rc_ecs.return_component(*entity, position);
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
		let mut matching_entities = rc_ecs.query(vec![TypeId::of::<PositionComp>()]);
		for entity in &matching_entities {
			let position = rc_ecs.get_component_ref::<PositionComp>(*entity).unwrap();
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
		let mut matching_entities = rc_ecs.query(vec![TypeId::of::<VelocityComp>()]);
		for entity in &matching_entities {
			let velocity = rc_ecs.get_component_ref::<VelocityComp>(*entity).unwrap();
			let index = expected_velocities
				.iter()
				.position(|x| *x == velocity.vel)
				.expect(format!("Vel was wrong! {}", velocity.vel).as_str());
			expected_velocities.remove(index);
		}
		assert_eq!(expected_velocities.len(), 0);
	}
	fn create_pv(rc_ecs: &mut RcEcs, pos: Point, vel: Point) -> Lentity {
		let mut entity = rc_ecs.create_entity();
		rc_ecs.add_component(entity, PositionComp { pos });
		rc_ecs.add_component(entity, VelocityComp { vel });
		rc_ecs.complete_entity(entity);
		entity
	}
	#[test]
	fn test_basic_dupey_processing() {
		let mut rc_ecs = RcEcs::new();
		create_pv(&mut rc_ecs, Point::new(0.0, 0.0), Point::new(1.0, 0.0));
		create_pv(&mut rc_ecs, Point::new(0.0, 0.0), Point::new(1.0, 0.0));
		create_pv(&mut rc_ecs, Point::new(0.0, 0.0), Point::new(0.0, 1.0));
		create_pv(&mut rc_ecs, Point::new(2.0, 0.0), Point::new(1.0, 0.0));
		create_pv(&mut rc_ecs, Point::new(2.0, 0.0), Point::new(0.0, 1.0));

		let matching_entities =
			rc_ecs.query_uniques(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);

		assert_eq!(matching_entities.len(), 4);
		for entity in &matching_entities {
			let mut position = rc_ecs.get_component::<PositionComp>(*entity).unwrap();
			let velocity = rc_ecs.get_component_ref::<VelocityComp>(*entity).unwrap();
			// Increment the position by the velocity
			position.pos += velocity.vel;
			rc_ecs.return_component(*entity, position);
		}
		// Assert that both entities have correct positions
		let mut matching_entities =
			rc_ecs.query(vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]);
		let mut expected_positions = vec![
			Point::new(1.0, 0.0),
			Point::new(1.0, 0.0),
			Point::new(0.0, 1.0),
			Point::new(3.0, 0.0),
			Point::new(2.0, 1.0),
		];
		for entity in &matching_entities {
			let position = rc_ecs.get_component_ref::<PositionComp>(*entity).unwrap();
			// Remove the position from the expected positions

			// println!("{}", &position.pos);
			let index = expected_positions
				.iter()
				.position(|x| *x == position.pos)
				.expect(format!("Position was wrong! {}", position.pos).as_str());
			expected_positions.remove(index);
		}
		assert_eq!(expected_positions.len(), 0);
	}

	#[test]
	fn test_indexing() {
		let mut rc_ecs = RcEcs::new();
		rc_ecs.create_index::<UniverseComp>();
		for i in 0..10 {
			let mut entity = rc_ecs.create_entity();
			rc_ecs.add_component(entity, PositionComp {
				pos: Point::new(2.0, i as f32),
			});
			rc_ecs.add_component(entity, UniverseComp { universe_id: i % 5 });
			rc_ecs.complete_entity(entity);
		}
		let index_results = rc_ecs.query_index::<UniverseComp>(3);
		assert_eq!(index_results.len(), 2);
		let index_results = rc_ecs.query_index::<UniverseComp>(1);
		assert_eq!(index_results.len(), 2);
	}
}
