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
    pub fn add_component(&mut self, data: T) -> ShelfRef{
        let shelf = Shelf {
            data,
            available_copy: None,
        };

        // hash the data
        let mut hasher = DefaultHasher::new();
        shelf.data.hash(&mut hasher);
        let hash = hasher.finish();

        let comp_index = self.vec.push(shelf);

        self.set.insert(hash, comp_index);
        ShelfRef {
            index: comp_index,
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