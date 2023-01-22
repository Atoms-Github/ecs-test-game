use crate::ui::ui_settings::GuiSettings;
use egui::Ui;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimSettings {
    pub universe_count: usize,
    pub entity_count: usize,
    pub brain_type: BrainType,
    pub challenge_type: Challenge,
    pub all_at_once: bool,
    pub rts_range: f32,
    pub paint_speed: f32,
}
impl SimSettings {
    pub fn draw(&mut self, ui: &mut Ui) {
        ui.label("Requested universe count");
        ui.add(egui::DragValue::new(&mut self.universe_count).speed(0.1));
        ui.label("Entity count");
        ui.add(egui::DragValue::new(&mut self.entity_count).speed(0.1));
        ui.label("All at once");
        ui.checkbox(&mut self.all_at_once, "All at once");

        let resp_brain = egui::ComboBox::from_label("Brain type")
            .selected_text(format!("{:?}", self.brain_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.brain_type, BrainType::Legion, "Legion");
                ui.selectable_value(&mut self.brain_type, BrainType::SqlDuck, "Sql duck");
                ui.selectable_value(&mut self.brain_type, BrainType::SqlIte, "Sqlite");
                ui.selectable_value(&mut self.brain_type, BrainType::SqlPostgres, "Postgres");
            })
            .response;
        let resp_challenge = egui::ComboBox::from_label("Challenge type")
            .selected_text(format!("{:?}", self.challenge_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.challenge_type,
                    Challenge::Rts {
                    },
                    "Rts",
                );
                ui.selectable_value(
                    &mut self.challenge_type,
                    Challenge::PaintClosest {},
                    "Get Nearest",
                );
                ui.selectable_value(
                    &mut self.challenge_type,
                    Challenge::SpacialArray,
                    "Spacial Array",
                );
            })
            .response;

        match &mut self.challenge_type {
            Challenge::Rts {} => {
                ui.label("Rts range");
                ui.add(egui::DragValue::new(&mut self.rts_range).speed(0.1));
            }
            Challenge::PaintClosest {} => {

                ui.label("Paint speed");
                ui.add(egui::DragValue::new(&mut self.paint_speed).speed(0.5));
            }
            Challenge::SpacialArray => {}
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BrainType {
    Legion,
    SqlDuck,
    SqlIte,
    SqlPostgres
}
impl fmt::Display for BrainType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Challenge {
    Rts,
    PaintClosest,
    SpacialArray,
}
impl Default for Challenge {
    fn default() -> Self {
        Challenge::PaintClosest {}
    }
}
impl Default for BrainType {
    fn default() -> Self {
        BrainType::SqlPostgres
    }
}
impl Default for SimSettings {
    fn default() -> Self {
        Self {
            universe_count: 1,
            entity_count: 100,
            brain_type: Default::default(),
            challenge_type: Default::default(),
            all_at_once: true,
            rts_range: 50.0,
            paint_speed: 10.0
        }
    }
}
