use ggez::input::mouse::position;
use rand::Rng;
use crate::brains::{Brain, SystemType};
use crate::challenges::Challenge;
use crate::{MAP_SIZE, Point};
#[derive(Clone)]
pub struct RtsChallenge {
    pub units_count: usize,
}
impl Challenge for RtsChallenge {
    fn init(&mut self, brain: &mut dyn Brain, universe_count: usize) {
        let mut rand = rand::thread_rng();

        const SPEED : f32 = 30.0;
        // Create units. Distribute them evenly across universes.
        let mut universe_id = 0;
        for _ in 0..self.units_count {
            let position = Point::new(rand.gen_range(0.0..MAP_SIZE), rand.gen_range(0.0..MAP_SIZE));
            let velocity = Point::new(rand.gen_range(-SPEED..SPEED), rand.gen_range(-SPEED..SPEED));
            let team = rand.gen_range(0..3);
            brain.add_entity_unit(position, velocity, team, universe_id);
            universe_id = (universe_id + 1) % universe_count;
        }
    }
    fn get_tick_systems(&self) -> Vec<SystemType> {
        return vec![SystemType::ACCELERATION, SystemType::VELOCITY
            , SystemType::MAP_EDGE, SystemType::UPDATE_TIMED_LIFE
            , SystemType::SHOOT, SystemType::DELETE_EXPIRED];
    }
    fn clone_box(&self) -> Box<dyn Challenge> {
        Box::new(self.clone())
    }
}
