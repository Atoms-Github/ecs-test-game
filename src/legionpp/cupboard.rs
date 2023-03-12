use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use crate::legionpp::unmoving_vec::UnmovingVec;
use crate::utils::HashMe;

#[derive(Clone)]
pub struct Cupboard<T: Clone> {
	vec: UnmovingVec<Shelf<T>>,
	pub hash_shelf_lookup: HashMap<u64, ShelfRef>,
}
impl<T: Clone + Hash + Debug> Cupboard<T> {
	pub fn new() -> Cupboard<T> {
		Cupboard {
			vec: UnmovingVec::new(),
			hash_shelf_lookup: HashMap::new(),
		}
	}

	pub fn add_qty(&mut self, shelf_ref: ShelfRef, qty: u32) {
		let shelf = self.vec.get_mut(shelf_ref).unwrap();
		match shelf {
			Shelf::One { data } => {
				let mut new_shelf = Shelf::Many {
					data_backup: data.clone().take().unwrap(),
					data: Some(Box::new(data.take().unwrap())),
					qty,
				};
				std::mem::swap(shelf, &mut new_shelf);
			}
			Shelf::Many {
				qty: existing_qty, ..
			} => {
				*existing_qty += qty;
			}
		}
	}

	pub fn add_component(&mut self, new_data: T) -> ShelfRef {
		// hash the data
		let hash = new_data.hash_me();

		let maybe_existing = self.hash_shelf_lookup.get(&hash);
		match maybe_existing {
			Some(existing) => {
				println!("(Found existing)");
				let shelf = self.vec.get_mut(*existing).unwrap();
				match shelf {
					Shelf::One {
						data: existing_data,
					} => {
						println!("=> Existing is One");
						let mut new_shelf = Shelf::Many {
							data_backup: existing_data.take().unwrap(),
							data:        Some(Box::new(new_data)),
							qty:         2,
						};
						std::mem::swap(shelf, &mut new_shelf);
						*existing
					}
					Shelf::Many {
						data_backup: data,
						data: available_copy,
						qty,
					} => {
						println!("=> Existing is currently Many was: {} now: {}", qty, *qty + 1);
						*qty += 1;
						*available_copy = Some(Box::new(new_data));
						*existing
					}
				}
			}
			None => {
				println!("(doesn't exist)");
				let shelf = Shelf::One {
					data: Some(new_data),
				};
				let comp_index = self.vec.push(shelf);

				self.hash_shelf_lookup.insert(hash, comp_index);
				comp_index
			}
		}
	}

	pub fn get_shelf_mut(&mut self, shelf_ref: &ShelfRef) -> &mut Shelf<T> {
		self.vec.get_mut(*shelf_ref).unwrap()
	}

	pub fn get_shelf(&self, shelf_ref: &ShelfRef) -> &Shelf<T> {
		self.vec.get(*shelf_ref).unwrap()
	}
}
#[derive(Clone)]
pub enum Shelf<T: Clone> {
	One {
		data: Option<T>,
	},
	Many {
		data_backup: T,
		data:        Option<Box<T>>,
		qty:         u32,
	},
}

pub type ShelfRef = usize;
