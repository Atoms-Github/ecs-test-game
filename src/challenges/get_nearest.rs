use ggez::graphics::Color;
use rand::Rng;
use crate::brains::{Brain, SystemType};
use crate::challenges::Challenge;
use crate::{MAP_SIZE, Point};
use crate::utils::GenRandom;


#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeGetNearest {
    pub units_count: usize,

}
impl Challenge for ChallengeGetNearest {
    fn init(&mut self, brain: &mut dyn Brain, universe_count: usize) {
        let mut rand = rand::thread_rng();
        const SPEED : f32 = 30.0;
        // Create units. Distribute them evenly across universes.
        let mut universe_id = 0;
        for _ in 0..self.units_count {
            let position = Point::new(rand.gen_range(0.0..MAP_SIZE), rand.gen_range(0.0..MAP_SIZE));
            brain.add_entity_positional_dummy(position, Color::gen_random());
        }
    }
    fn get_tick_systems(&self) -> Vec<SystemType> {
        return vec![SystemType::PaintNearest]
    }
    fn clone_box(&self) -> Box<dyn Challenge> {
        Box::new(self.clone())
    }
}
