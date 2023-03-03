use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::hash::{Hash, Hasher};
use trait_bound_typemap::{CloneTypeMap, AnyTypeMap, TypeMap, TypeMapKey};
use crate::legionpp::cupboard::Cupboard;

pub struct Lpp {
    pub cupboards: CloneTypeMap
}

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
}

#[cfg(test)]
mod tests {
    use plotters::style::text_anchor::Pos;
    use super::*;
    use crate::brains::com::*;
    use crate::legionpp::utils::gett;
    use crate::Point;

    #[test]
    fn basic() {
        let mut lpp = Lpp::new();
        let position_comp = PositionComp{pos: Point::new(0.0, 0.0)};
        let mut entity = lpp.create_entity();
        lpp.add_component(entity, position_comp);

        let velocity_comp = VelocityComp{vel: Point::new(0.0, 0.0)};
        lpp.add_component(entity, velocity_comp);

        // Query for all entities with a position component
        let mut matching_entities = lpp.query(vec![gett::<PositionComp>(), gett::<VelocityComp>()]);
        for entity in matching_entities {
            let position = lpp.get_component::<PositionComp>(entity);
            let velocity = lpp.get_component_ref::<VelocityComp>(entity);
            // Increment the position by the velocity
            position.pos += velocity.vel;
            println!("Entity {:?} has position {:?} and velocity {:?}", entity, position, velocity);
            lpp.return_component(entity, position);
        }

    }
}

