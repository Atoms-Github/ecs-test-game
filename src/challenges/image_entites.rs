use duckdb::ffi::rand;
use ggez::graphics::Color;
use ggez::input::mouse::position;
use ggez::winit::dpi::Position;
use glam::Vec2;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};

use crate::brains::{Brain, SystemType};
use crate::challenges::ChallengeTrait;
use crate::simulation_settings::SimSettings;
use crate::ui::ui_settings::GuiSettings;
use crate::utils::GenRandom;
use crate::{utils, Point, MAP_SIZE};

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeBlob {}
impl ChallengeTrait for ChallengeBlob {
	fn init(&mut self, brain: &mut dyn Brain, universe_count: usize, settings: &SimSettings) {
		for i in 0..settings.entity_count {
			let mut blob = vec![2; 100];
			brain.add_entity_blob(Point::new(20.0, 30.0), blob, 1.0);
		}
	}

	fn get_tick_systems(&self) -> Vec<SystemType> {
		return vec![];
	}

	fn clone_box(&self) -> Box<dyn ChallengeTrait> {
		Box::new(self.clone())
	}
}
