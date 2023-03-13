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

pub fn edit_image(blob: &mut Vec<u8>) {
	if blob[0] == 255 {
		for i in 0..blob.len() {
			if i % 4 == 0 {
				blob[i] = 0;
			}
		}
	} else {
		for i in 0..blob.len() {
			if i % 4 == 0 {
				blob[i] = 255;
			}
		}
	}
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChallengeImageEditing {}
impl ChallengeTrait for ChallengeImageEditing {
	fn init(
		&mut self,
		ctx: &mut Context,
		brain: &mut dyn Brain,
		universe_count: usize,
		settings: &SimSettings,
	) {
		let image_paths = vec!["plantssmol.jpg"];
		// let image_paths = vec!["plants.jpg", "shark.jpg", "stars.jpg"];

		let mut images = Vec::new();

		for image in &image_paths {
			let image = ggez::graphics::Image::new(ctx, format!("/{}", image)).unwrap();
			println!("Loaded image");
			let blob = image.to_rgba8(ctx).unwrap();
			images.push(BlobComp { blob });
		}

		for i in 0..settings.entity_count / images.len() {
			for blob in &images {
				brain.add_entity_blob(Point::new(100.0 * i as f32, 20.0), blob, 1.0, Some(i % 2));
			}
		}
	}

	fn get_tick_systems(&self) -> Vec<SystemType> {
		return vec![SystemType::EditTeamOneImage];
	}

	fn clone_box(&self) -> Box<dyn ChallengeTrait> {
		Box::new(self.clone())
	}
}
