use egui::Ui;
use ggez::GameResult;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuiSettings {
    pub shoot_distance: f32,
    pub view_universe: usize,
    pub universe_count: usize,
    pub entity_count: usize,
    pub blend_speed: f32,
    pub brain_type: BrainType,
    pub challenge_type: ChallengeType,
    pub all_at_once: bool,
}
impl GuiSettings {
    pub fn new() -> GuiSettings {
        GuiSettings {
            shoot_distance: 10.0,
            view_universe: 0,
            universe_count: 1,
            blend_speed: 10.0,
            entity_count: 5,
            brain_type: BrainType::SqlIte,
            challenge_type: ChallengeType::Rts,
            all_at_once: true,
        }
    }
    pub fn draw(&mut self, ui: &mut Ui) {
        ui.label("Meet distance");
        ui.add(egui::DragValue::new(&mut self.shoot_distance).speed(0.1));
        ui.label("Universe");
        ui.add(egui::DragValue::new(&mut self.view_universe).speed(0.1));
        ui.label("Requested universe count");
        ui.add(egui::DragValue::new(&mut self.universe_count).speed(0.1));
        ui.label("Blend Speed");
        ui.add(egui::DragValue::new(&mut self.blend_speed).speed(0.5));
        ui.label("Entity count");
        ui.add(egui::DragValue::new(&mut self.entity_count).speed(0.1));
        ui.label("All at once");
        ui.checkbox(&mut self.all_at_once, "All at once");

        // Add ui for self.universe_count:

        let resp_brain = egui::ComboBox::from_label("Brain type")
            .selected_text(format!("{:?}", self.brain_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.brain_type, BrainType::Legion, "Legion");
                ui.selectable_value(&mut self.brain_type, BrainType::SqlDuck, "Sql duck");
                ui.selectable_value(&mut self.brain_type, BrainType::SqlIte, "Sqlite");
            })
            .response;
        let resp_challenge = egui::ComboBox::from_label("Challenge type")
            .selected_text(format!("{:?}", self.challenge_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.challenge_type, ChallengeType::Rts, "Rts");
                ui.selectable_value(
                    &mut self.challenge_type,
                    ChallengeType::GetNearest,
                    "Get Nearest",
                );
                ui.selectable_value(
                    &mut self.challenge_type,
                    ChallengeType::SpacialArray,
                    "Spacial Array",
                );
            })
            .response;
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BrainType {
    Legion,
    SqlDuck,
    SqlIte,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChallengeType {
    Rts,
    GetNearest,
    SpacialArray,
}
