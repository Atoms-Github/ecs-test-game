use egui::Ui;
use ggez::GameResult;

use crate::simulation_settings::{BrainType, Challenge, SimSettings};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuiSettings {
	pub view_universe:       usize,
	pub simulation_settings: SimSettings,
	pub image_scale:         f32,
	pub image_offset:        f32,
}

impl GuiSettings {
	pub fn draw(&mut self, ui: &mut Ui) {
		ui.label("View universe");
		ui.add(egui::DragValue::new(&mut self.view_universe).speed(0.1));
		ui.label("Image scale");
		ui.add(egui::DragValue::new(&mut self.image_scale).speed(0.001));
		ui.label("Image offset");
		ui.add(egui::DragValue::new(&mut self.image_offset).speed(0.03));

		self.simulation_settings.draw(ui);
	}

	pub fn new() -> Self {
		Self {
			view_universe:       0,
			simulation_settings: SimSettings::default(),
			image_scale:         2.3,
			image_offset:        0.0,
		}
	}
}

impl Default for GuiSettings {
	fn default() -> Self {
		Self::new()
	}
}
