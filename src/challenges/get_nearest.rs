use crate::brains::{Brain, SystemType};
use crate::challenges::Challenge;
use crate::ui::ui_settings::GuiSettings;
use crate::utils::GenRandom;
use crate::{Point, MAP_SIZE};
use ggez::graphics::Color;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeGetNearest {}
impl Challenge for ChallengeGetNearest {
    fn init(&mut self, brain: &mut dyn Brain, universe_count: usize, settings: &GuiSettings) {
        let mut rand = rand::thread_rng();
        const SPEED: f32 = 30.0;
        // Create units. Distribute them evenly across universes.
        let mut universe_id = 0;
        for _ in 0..settings.entity_count {
            let position = Point::new(rand.gen_range(0.0..MAP_SIZE), rand.gen_range(0.0..MAP_SIZE));
            let blue = rand.gen_range(0.0..1.0);
            brain.add_entity(position, None, blue)
        }
    }
    fn get_tick_systems(&self) -> Vec<SystemType> {
        return vec![SystemType::PaintNearest];
    }
    fn clone_box(&self) -> Box<dyn Challenge> {
        Box::new(self.clone())
    }
}
