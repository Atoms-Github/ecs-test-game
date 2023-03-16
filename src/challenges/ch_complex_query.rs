use duckdb::ffi::rand;
use ggez::graphics::Color;
use ggez::input::mouse::position;
use ggez::winit::dpi::Position;
use ggez::Context;
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
pub struct ChallengeQuery {}
impl ChallengeTrait for ChallengeQuery {
	fn init(
		&mut self,
		ctx: &mut Context,
		brain: &mut dyn Brain,
		universe_count: usize,
		settings: &SimSettings,
	) {
		for i in 0..settings.entity_count * 5_000 {
			brain.add_entity_unit(Point::new(0.0, 0.0), Point::new(0.0, 0.0), 0, 0);
		}
		for i in 0..settings.entity_count {
			brain.add_entity_unit(Point::new(0.0, 0.0), Point::new(0.0, 0.0), 1, 0);
		}
	}

	fn get_tick_systems(&self) -> Vec<SystemType> {
		return vec![SystemType::EditTeamOneColor];
	}

	fn clone_box(&self) -> Box<dyn ChallengeTrait> {
		Box::new(self.clone())
	}
}
