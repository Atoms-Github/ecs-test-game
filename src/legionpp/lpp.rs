use std::any::TypeId;
use std::borrow::Borrow;
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::hash::{Hash, Hasher};
use ggez::filesystem::create;
use legion::Entity;
use trait_bound_typemap::{CloneTypeMap, AnyTypeMap, TypeMap, TypeMapKey};
use crate::legionpp::cupboard::{Cupboard, Shelf, ShelfRef};
pub type TypeSig = BTreeSet<TypeId>;

pub struct Lpp {
    pub cupboards: CloneTypeMap,
    pub lentities: HashMap<Lentity, InternalEntity>,
    pub archetypes: HashMap<TypeSig, Vec<Lentity>>,
}
pub struct InternalEntity {
    pub shelves: HashMap<TypeId, ShelfRef>,

}
pub type Lentity = usize;

impl <T : 'static + Clone> TypeMapKey for OurKey<T>{
    type Value = T;
}
pub struct OurKey<T>{
    _t: T
}
impl Lpp {
    pub fn add<T : Clone + 'static>(&mut self, cupboard: Cupboard<T>) {
        self.cupboards.insert::<OurKey<Cupboard<T>>>(cupboard);
    }
    pub fn new() -> Lpp {
        Lpp{
            cupboards: CloneTypeMap::new(),
            lentities: Default::default(),
            archetypes: Default::default(),
        }
    }
    pub fn create_entity(&mut self) -> Lentity {
        let lentity = self.lentities.len();
        self.lentities.insert(lentity, InternalEntity{shelves: Default::default()});
        return lentity;
    }
    fn create_cupboard_if_needed<T : Clone + Hash + 'static>(&mut self) {
        if self.cupboards.get::<OurKey<Cupboard<T>>>().is_none() {
            self.add(Cupboard::<T>::new());
        }
    }

    pub fn get_entity(&mut self, lentity: Lentity) -> &mut InternalEntity {
        self.lentities.get_mut(&lentity).expect("Ent doesn't exist")
    }

    pub fn add_component<T : Clone + Hash + 'static>(&mut self, lentity: Lentity, component: T) {
        self.create_cupboard_if_needed::<T>();
        let mut cupboard = self.cupboards.get_mut::<OurKey<Cupboard<T>>>().unwrap();
        let shelf_ref = cupboard.add_component(component);
        self.get_entity(lentity).shelves.insert(TypeId::of::<T>(), shelf_ref);
    }

    pub fn complete_entity(&mut self, lentity: Lentity) {
        let type_sig = self.get_entity(lentity).shelves.keys().cloned().collect();
        self.archetypes.entry(type_sig).or_insert_with(|| Vec::new()).push(lentity);
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

    pub fn get_component_ref<T : Clone + Hash + 'static>(&mut self, lentity: Lentity) -> Option<&T> {
        let cupboard = self.cupboards.get_mut::<OurKey<Cupboard<T>>>()?;
        let shelf_ref = self.lentities
            .get_mut(&lentity).expect("Ent doesn't exist")
            .shelves.get(&TypeId::of::<T>())?;
        let shelf = cupboard.get_shelf(shelf_ref);
        match shelf{
            Shelf::One { data } => {
                Some(data.as_ref().unwrap())
            }
            Shelf::Many { data, .. } => {
                Some(data)
            }
        }
    }

    pub fn get_component<T : Clone + Hash + 'static>(&mut self, lentity: Lentity) -> Option<T> {
        let cupboard = self.cupboards.get_mut::<OurKey<Cupboard<T>>>()?;
        let shelf_ref = self.lentities
            .get_mut(&lentity).expect("Ent doesn't exist")
            .shelves.get(&TypeId::of::<T>())?;
        let shelf = cupboard.get_shelf(shelf_ref);

        match shelf{
            Shelf::One { data } => {
                assert!(data.is_some(), "Was it already on loan?");
                data.take()
            }
            Shelf::Many { data, available_copy, qty } => {
                let shelf_data = available_copy.take();
                if let Some(on_loan) = shelf_data {
                    return Some(*on_loan);
                }
                Some(data.clone())
            }
        }

    }

    pub fn return_component<T : Clone + Hash + 'static>(&mut self, lentity: Lentity, component: T) {
        let cupboard = self.cupboards.get_mut::<OurKey<Cupboard<T>>>().unwrap();
        let shelf_ref = self.lentities
            .get_mut(&lentity).expect("Ent doesn't exist")
            .shelves.get(&TypeId::of::<T>()).unwrap();
        let shelf = cupboard.get_shelf(shelf_ref);
        match shelf{
            Shelf::One { data } => {
                *data = Some(component);
            }
            Shelf::Many { data, available_copy, qty } => {
                *available_copy = Some(Box::new(component));
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
        let position_comp = PositionComp{pos: Point::new(0.0, 0.0)};
        let mut entity = lpp.create_entity();
        lpp.add_component(entity, position_comp);

        let velocity_comp = VelocityComp{vel: Point::new(0.0, 0.0)};
        lpp.add_component(entity, velocity_comp);

        lpp.complete_entity(entity);

        // Query for all entities with a position component
        let mut matching_entities = lpp.query(
            vec![TypeId::of::<PositionComp>(), TypeId::of::<VelocityComp>()]
        );

        for entity in matching_entities {
            let mut position = lpp.get_component::<PositionComp>(entity).unwrap();
            let velocity = lpp.get_component_ref::<VelocityComp>(entity).unwrap();
            // Increment the position by the velocity
            position.pos += velocity.vel;
            println!("Entity {:?} has position {:?} and velocity {:?}", entity, position, velocity);
            lpp.return_component(entity, position);
        }

    }
}

