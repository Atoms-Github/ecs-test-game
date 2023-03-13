use std::{env, path};

use duckdb::ffi::rand;
use ggez::graphics::Color;
use ggez::input::mouse::position;
use ggez::winit::dpi::Position;
use ggez::Context;
use glam::Vec2;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{thread_rng, Rng, SeedableRng};

use crate::brains::com::BlobComp;
use crate::brains::{Brain, SystemType};
use crate::challenges::ChallengeTrait;
use crate::simulation_settings::SimSettings;
use crate::ui::ui_settings::GuiSettings;
use crate::utils::GenRandom;
use crate::{utils, Point, MAP_SIZE};

#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeSlideshow {}
impl ChallengeTrait for ChallengeSlideshow {
	fn init(
		&mut self,
		ctx: &mut Context,
		brain: &mut dyn Brain,
		universe_count: usize,
		settings: &SimSettings,
	) {
		let image_paths = vec!["plants.jpg", "shark.jpg", "stars.jpg"];

		let mut images = Vec::new();

		for image in &image_paths {
			let image = ggez::graphics::Image::new(ctx, format!("/{}", image)).unwrap();

			let blob = image.to_rgba8(ctx).unwrap();
			images.push(BlobComp { blob });
		}

		for i in 0..settings.entity_count / images.len() {
			for blob in &images {
				brain.add_entity_blob(Point::new(20.0, 20.0), blob, 1.0, None);
			}
		}
	}

	fn get_tick_systems(&self) -> Vec<SystemType> {
		return vec![];
	}

	fn clone_box(&self) -> Box<dyn ChallengeTrait> {
		Box::new(self.clone())
	}
}
