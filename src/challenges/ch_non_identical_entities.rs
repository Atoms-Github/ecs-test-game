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
pub struct ChallengeNonIdenticalEntities {}
impl ChallengeTrait for ChallengeNonIdenticalEntities {
	fn init(
		&mut self,
		ctx: &mut Context,
		brain: &mut dyn Brain,
		universe_count: usize,
		settings: &SimSettings,
	) {
		for i in 0..settings.entity_count {
			let mut rng = thread_rng();
			let mut pos =
				Point::new(rng.gen_range(0.0..(MAP_SIZE as f32)), rng.gen_range(0.0..(MAP_SIZE as f32)));
			let mut vel = Point::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
			brain.add_entity(pos, Some(vel), 1.0);
		}
	}

	fn get_tick_systems(&self) -> Vec<SystemType> {
		return vec![SystemType::Velocity];
	}

	fn clone_box(&self) -> Box<dyn ChallengeTrait> {
		Box::new(self.clone())
	}
}
