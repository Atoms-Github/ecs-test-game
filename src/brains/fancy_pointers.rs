use crate::brains::com::ExportEntity;
use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;
use crate::Point;

struct HoleyPointers {}
impl Brain for HoleyPointers {
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    ) {
        todo!()
    }

    fn add_entity(&mut self, position: Point, velocity: Option<Point>, blue: f32) {
        todo!()
    }

    fn get_entities(&mut self, universe_id: usize) -> Vec<ExportEntity> {
        todo!()
    }

    fn init(&mut self, systems: &Vec<SystemType>) {
        todo!()
    }

    fn tick_systems(&mut self, delta: f32, settings: &GuiSettings, systems: &Vec<SystemType>) {
        todo!()
    }

    fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &GuiSettings) {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }
}
