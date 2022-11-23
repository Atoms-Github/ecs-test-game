use crate::brains::{Brain, SystemType};

mod rts;

pub trait Challenge {
    fn init<B: Brain>(&mut self, brain: &mut B);
    fn get_tick_systems() -> Vec<SystemType>;
}
