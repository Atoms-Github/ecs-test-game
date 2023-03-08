use crate::brains::{Brain, SystemType};
use crate::simulation_settings::SimSettings;
use crate::ui::ui_settings::GuiSettings;

pub mod get_nearest;
pub mod identical_entities;
pub mod image_entites;
pub mod rts;
pub mod spacial_array;

pub trait ChallengeTrait {
	fn init(&mut self, brain: &mut dyn Brain, universe_count: usize, settings: &SimSettings);
	fn get_tick_systems(&self) -> Vec<SystemType>;
	fn clone_box(&self) -> Box<dyn ChallengeTrait>;
}
