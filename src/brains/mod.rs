use std::fmt;
use std::time::Duration;

use ggez::graphics::Color;

use crate::ui::ui_settings::GuiSettings;
use crate::Point;

pub mod com;
pub mod ecs_concepts;
pub mod legion_scheduled;
pub mod legion_sequential;
pub mod sql_brains;
pub mod sql_flat_table;

pub trait Brain {
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    );
    fn add_entity(&mut self, position: Point, velocity: Option<Point>, color: Color);

    fn get_entities(&mut self, universe_id: usize) -> Vec<(Point, Color)>;

    fn init_systems(&mut self, systems: &Vec<SystemType>);

    fn get_tick_all_at_once(&self) -> bool;
    fn tick_systems(&mut self, delta: f32, settings: &GuiSettings);
    fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &GuiSettings);

    fn get_name(&self) -> String;
}
#[derive(Debug, PartialEq, Clone)]
pub enum SystemType {
    Velocity,
    Acceleration,
    MapEdge,
    UpdateTimedLife,
    Shoot,
    DeleteExpired,
    PaintNearest,
}
impl SystemType {
    pub fn get_name(&self) -> String {
        return format!("{:?}", self);
    }
}
