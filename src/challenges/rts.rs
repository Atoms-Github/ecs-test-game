use crate::brains::{Brain, SystemType};
use crate::challenges::Challenge;

pub struct RtsChallenge {}
impl Challenge for RtsChallenge {
    fn init<B: Brain>(&mut self, brain: &mut B) {
        todo!()
    }

    fn get_tick_systems(&self) -> Vec<SystemType> {
        todo!()
    }
}
