use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;

pub mod get_nearest;
pub mod rts;
pub mod spacial_array;

pub trait ChallengeTrait {
    fn init(&mut self, brain: &mut dyn Brain, universe_count: usize, settings: &GuiSettings);
    fn get_tick_systems(&self) -> Vec<SystemType>;
    fn clone_box(&self) -> Box<dyn ChallengeTrait>;
}
