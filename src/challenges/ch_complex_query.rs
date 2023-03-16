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
		let mut target_team = (settings.entity_count / 1000).max(1);
		if settings.entity_count == 0 {
			target_team = 0;
		}
		for i in 0..target_team {
			brain.add_entity_unit(Point::new(0.0, 0.0), Point::new(0.0, 0.0), 1, 0);
		}
		let non_target_team = settings.entity_count - target_team;
		for i in 0..non_target_team {
			brain.add_entity_unit(Point::new(0.0, 0.0), Point::new(0.0, 0.0), 0, 0);
		}
	}

	fn get_tick_systems(&self) -> Vec<SystemType> {
		return vec![SystemType::EditTeamOneColor];
	}

	fn clone_box(&self) -> Box<dyn ChallengeTrait> {
		Box::new(self.clone())
	}
}
