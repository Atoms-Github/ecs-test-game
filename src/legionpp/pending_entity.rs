use std::collections::{BTreeMap, BTreeSet};
use serde::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::comp_store::TypesSet;
use super::comp_store::SingleComp;
use crate::bblocky::super_any::SuperAny;
use std::fmt::Debug;
use serde::de::DeserializeOwned;
use crate::EcsConfig;
use crate::utils::{TypeIdNum, gett};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PendingEntity{
    data: BTreeMap<TypeIdNum, SingleComp>,
}

impl PendingEntity {
    pub fn new() -> Self{
        Self::default()
    }
    pub fn hash_types(&self) -> TypesSet {
        let mut types = BTreeSet::new();
        for new_type in self.data.keys(){
            types.insert(*new_type);
        }
        return types;
    }
    pub fn post_deserialize(&mut self, config: &EcsConfig){
        for(a,b) in &mut self.data{
            b.list.post_deserialize(&config);
        }
    }
    pub fn iter(&self) -> std::collections::btree_map::Iter<TypeIdNum, SingleComp>{ // Optimum make it return move instead of reference (then clone).
        return self.data.iter();
    }
    pub fn add_comp<T : 'static + Serialize + Clone + DeserializeOwned + Send + Debug>(&mut self, value: T) {
        assert!(self.set_comp(value).is_none(), "Pending entity already contained that component type!");

        // assert!(gett::<$type_name>() != 5434297715843079649, "Item: {}", std::any::type_name::<$type_name>());
    }
    pub fn set_comp<T : 'static + Serialize + Clone + DeserializeOwned + Send + Debug>(&mut self, value: T) -> Option<SingleComp> {
        let bytes = unsafe {crate::unsafe_utils::struct_as_u8_slice(&value)}.to_vec();
        return self.data.insert(crate::utils::gett::<T>(), SuperAny::new(value));
    }
    pub fn remove<T : 'static + Serialize + Clone + DeserializeOwned + Send + Debug>(&mut self) {
        self.data.remove(&crate::utils::gett::<T>());
    }
    pub fn get_mut<T : 'static + Serialize + Clone + DeserializeOwned + Send + Debug>(&mut self) -> Option<&mut T>{
        self.data.get_mut(&gett::<T>()).map(|item|{item.get_mut::<T>()})
    }
}

