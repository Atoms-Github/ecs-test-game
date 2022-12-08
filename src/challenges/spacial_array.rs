use crate::brains::{Brain, SystemType};
use crate::challenges::Challenge;
use crate::ui::ui_settings::GuiSettings;
use crate::utils::GenRandom;
use crate::{utils, Point, MAP_SIZE};
use ggez::graphics::Color;
use ggez::winit::dpi::Position;
use glam::Vec2;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeSpatialArray {
    pub has_velocity_fraction: f64,
    pub dupe_entity_fraction: f64,
    pub unique_velocity_fraction: f64,
}
impl Challenge for ChallengeSpatialArray {
    fn init(&mut self, brain: &mut dyn Brain, universe_count: usize, settings: &GuiSettings) {
        let mut rand = rand::thread_rng();
        const SPEED: f32 = 30.0;
        // Create units. Distribute them evenly across universes.
        let mut universe_id = 0;

        let mut random = StdRng::seed_from_u64(10);

        let mut existing_entities: Vec<(Point, Option<Point>, Color)> = vec![];
        let mut existing_velocities = vec![];
        let unique_velocities =
            (self.unique_velocity_fraction * settings.entity_count as f64) as usize;
        while existing_entities.len() < settings.entity_count {
            if random.gen_bool(self.dupe_entity_fraction) && !existing_entities.is_empty() {
                let (position, velocity, colour) =
                    existing_entities.choose(&mut random).unwrap().clone();
                brain.add_entity(position, velocity, colour);
            } else {
                let position = Point::gen_random() * MAP_SIZE;
                let color = Color::gen_random();
                let mut velocity = None;
                if random.gen_bool(self.has_velocity_fraction) {
                    if existing_velocities.len() < unique_velocities {
                        let genned_velocity = Vec2::gen_random() * SPEED;
                        existing_velocities.push(genned_velocity);
                        velocity = Some(genned_velocity);
                    } else {
                        velocity = Some(*existing_velocities.choose(&mut random).clone().unwrap());
                    };
                }
                brain.add_entity(position, velocity, color);
                existing_entities.push((position, velocity, color));
            }
        }
    }
    fn get_tick_systems(&self) -> Vec<SystemType> {
        return vec![SystemType::Velocity];
    }
    fn clone_box(&self) -> Box<dyn Challenge> {
        Box::new(self.clone())
    }
}
