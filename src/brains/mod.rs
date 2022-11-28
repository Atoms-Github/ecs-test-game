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

pub trait Brain {
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    );
    fn add_entity_vel_dot(&mut self, position: Point, velocity: Point);
    fn add_entity_positional_dummy(&mut self, position: Point);

    fn get_entities(&mut self, universe_id: usize) -> Vec<(Point, Color)>;

    fn init_systems(&mut self, systems: &Vec<SystemType>);

    fn get_tick_all_at_once(&self) -> bool;
    fn tick_systems(&mut self, delta: f32, settings: GuiSettings);
    fn tick_system(&mut self, system: &SystemType, delta: f32);

    fn get_name(&self) -> String;
}
#[derive(Debug, PartialEq, Clone)]
pub enum SystemType {
    VELOCITY,
    ACCELERATION,
    MAP_EDGE,
    UPDATE_TIMED_LIFE,
    SHOOT,
    DELETE_EXPIRED,
}
impl SystemType {
    pub fn as_string(&self) -> &'static str {
        match self {
            SystemType::VELOCITY => "velocity",
            SystemType::MAP_EDGE => "map_edge",
            SystemType::UPDATE_TIMED_LIFE => "update_timed_life",
            SystemType::SHOOT => "shoot",
            SystemType::DELETE_EXPIRED => "delete_expired",
            SystemType::ACCELERATION => "acceleration",
        }
    }
}
