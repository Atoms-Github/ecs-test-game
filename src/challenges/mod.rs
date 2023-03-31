use ggez::Context;

use crate::brains::{Brain, SystemType};
use crate::simulation_settings::SimSettings;
use crate::ui::ui_settings::GuiSettings;

pub mod ch_complex_query;
pub mod ch_identical_entities;
pub mod ch_image_editing;
pub mod ch_image_slideshow;
pub mod ch_non_identical_entities;
pub mod ch_paint_closest;
pub mod ch_spatial_array;
pub mod ch_units_shooting;

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
