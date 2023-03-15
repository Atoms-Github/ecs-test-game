use ggez::Context;

use crate::brains::{Brain, SystemType};
use crate::simulation_settings::SimSettings;
use crate::ui::ui_settings::GuiSettings;

pub mod get_nearest;
pub mod identical_entities;
pub mod image_editing;
pub mod image_slideshow;
pub mod query_challenge;
pub mod rts;
pub mod spacial_array;

pub trait ChallengeTrait {
	fn init(
		&mut self,
		ctx: &mut Context,
		brain: &mut dyn Brain,
		universe_count: usize,
		settings: &SimSettings,
	);
	fn get_tick_systems(&self) -> Vec<SystemType>;
	fn clone_box(&self) -> Box<dyn ChallengeTrait>;
}
