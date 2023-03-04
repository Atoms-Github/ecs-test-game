use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::legionpp::unmoving_vec::UnmovingVec;

#[derive(Clone)]
pub struct Cupboard<T : Clone>{
    vec: UnmovingVec<Shelf<T>>,
    set: HashMap<u64, usize>,
}
impl<T: Clone + Hash> Cupboard<T>{
    pub fn new() -> Cupboard<T> {
        Cupboard{
            vec: UnmovingVec::new(),
            set: HashMap::new(),
        }
    }
    pub fn add_component(&mut self, new_data: T) -> ShelfRef{

        // hash the data
        let mut hasher = DefaultHasher::new();
        new_data.hash(&mut hasher);
        let hash = hasher.finish();

        let maybe_existing = self.set.get(&hash);
        match maybe_existing {
            Some(existing) => {
                let shelf = self.vec.get_mut(*existing).unwrap();
                match shelf {
                    Shelf::One { data: existing_data } => {
                        let mut new_shelf = Shelf::Many {
                            data: existing_data.take().unwrap(),
                            available_copy: Some(Box::new(new_data)),
                            qty: 2,
                        };
                        std::mem::swap(shelf, &mut new_shelf);
                        ShelfRef {
                            index: *existing,
                        }
                    },
                    Shelf::Many { data, available_copy, qty } => {
                        *qty += 1;
                        *available_copy = Some(Box::new(new_data));
                        ShelfRef {
                            index: *existing,
                        }
                    }
                }
            },
            None => {
                let shelf = Shelf::One{
                    data: Some(new_data),
                };
                let comp_index = self.vec.push(shelf);

                self.set.insert(hash, comp_index);
                ShelfRef {
                    index: comp_index,
                }
            }
        }
    }

    pub fn get_shelf(&mut self, shelf_ref: &ShelfRef) -> &mut Shelf<T> {
        self.vec.get_mut(shelf_ref.index).unwrap()
    }
}
#[derive(Clone)]
pub enum Shelf<T : Clone>{
    One{data: Option<T>},
    Many{data: T, available_copy: Option<Box<T>>, qty: u32},
}


pub struct ShelfRef{
    pub index: usize,
}