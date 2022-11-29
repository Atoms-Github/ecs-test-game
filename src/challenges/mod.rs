use crate::brains::{Brain, SystemType};

pub mod rts;
pub mod get_nearest;


pub trait Challenge {
    fn init(&mut self, brain: &mut dyn Brain, universe_count: usize);
    fn get_tick_systems(&self) -> Vec<SystemType>;
    fn clone_box(&self) -> Box<dyn Challenge>;
}
