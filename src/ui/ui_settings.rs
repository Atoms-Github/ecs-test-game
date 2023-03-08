use egui::Ui;
use ggez::GameResult;

use crate::simulation_settings::{BrainType, Challenge, SimSettings};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuiSettings {
	pub view_universe:       usize,
	pub simulation_settings: SimSettings,
}

impl GuiSettings {
	pub fn draw(&mut self, ui: &mut Ui) {
		ui.label("View universe");
		ui.add(egui::DragValue::new(&mut self.view_universe).speed(0.1));
		self.simulation_settings.draw(ui);
	}

	pub fn new() -> Self {
		Self {
			view_universe:       0,
			simulation_settings: SimSettings::default(),
		}
	}
}

impl Default for GuiSettings {
	fn default() -> Self {
		Self {
			view_universe:       0,
			simulation_settings: Default::default(),
		}
	}
}
