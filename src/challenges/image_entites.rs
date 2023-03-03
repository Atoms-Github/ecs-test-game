use duckdb::ffi::rand;
use crate::brains::{Brain, SystemType};
use crate::challenges::ChallengeTrait;
use crate::ui::ui_settings::GuiSettings;
use crate::utils::GenRandom;
use crate::{utils, Point, MAP_SIZE};
use ggez::graphics::Color;
use ggez::input::mouse::position;
use ggez::winit::dpi::Position;
use glam::Vec2;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};
use crate::simulation_settings::SimSettings;

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeIdenticalEntities {
}
impl ChallengeTrait for ChallengeIdenticalEntities {
    fn init(&mut self, brain: &mut dyn Brain, universe_count: usize, settings: &SimSettings) {
        for i in 0..settings.entity_count{
            brain.add_entity(
                Point::new(20.0, 20.0),
                Some(Point::new(2.0, 1.0)),
                0.7,
            );
        }
    }
    fn get_tick_systems(&self) -> Vec<SystemType> {
        return vec![SystemType::Velocity];
    }
    fn clone_box(&self) -> Box<dyn ChallengeTrait> {
        Box::new(self.clone())
    }
}
