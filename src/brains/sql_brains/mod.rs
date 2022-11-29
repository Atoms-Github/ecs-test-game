pub mod duckdb;

use std::backtrace::Backtrace;
use ggez::graphics::Color;
use crate::brains::{Brain, SystemType};
use crate::brains::com::ExportEntity;
use crate::Point;
use crate::ui::ui_settings::GuiSettings;
use crate::utils::FromTeam;

pub struct BrainDatabase<T> {
    data: T
}


impl<T : SqlBrain> BrainDatabase<T>{
    pub fn new() -> Self {
        Self {
            data: T::new()
        }
    }
}
pub trait SqlBrain{
    type PreppedStatement;
    fn new() -> Self;
    fn execute(&mut self, statement: &str, params: Vec<f32>);
    fn get_entities(&mut self, universe_id: usize) -> Vec<ExportEntity>;
}
impl<T : SqlBrain> Brain for BrainDatabase<T> {
    fn add_entity_unit(&mut self, position: Point, velocity: Point, team: usize, universe_id: usize) {
        let red = Color::from_team(team).r;
        self.data.execute("INSERT INTO entities (position_x, position_y, velocity_x, velocity_y, team, universe_id, color_r) VALUES (?, ?, ?, ?, ?, ?, ?)",
                          vec![position.x, position.y, velocity.x, velocity.y, team as f32, universe_id as f32, team as f32]);
    }

    fn add_entity_vel_dot(&mut self, position: Point, velocity: Point) {
    }

    fn add_entity_positional_dummy(&mut self, position: Point, color: Color) {
    }

    fn get_entities(&mut self, universe_id: usize) -> Vec<(Point, Color)> {
        return self.data.get_entities(universe_id).iter().map(|e| (e.pos, e.color)).collect();
    }

    fn init_systems(&mut self, systems: &Vec<SystemType>) {
        self.data.execute(                "CREATE TABLE entities (
            position_x REAL,
            position_y REAL,
            velocity_x REAL,
            velocity_y REAL,
            acceleration_x REAL,
            acceleration_y REAL,
            color_r REAL,
            team INTEGER,
            universe_id INTEGER,
            shooter_cooldown REAL,
            timed_life REAL
        )", vec![]);
    }

    fn get_tick_all_at_once(&self) -> bool {
        false
    }

    fn tick_systems(&mut self, delta: f32, settings: &GuiSettings) {
        panic!("Not meant to be called");
    }
    fn tick_system(&mut self, system: &SystemType, delta: f32) {
        match system{
            SystemType::Velocity => {
                self.data.execute("UPDATE entities SET position_x = position_x + velocity_x * ?;", vec![delta]);
                self.data.execute("UPDATE entities SET position_y = position_y + velocity_y * ?;", vec![delta]);
            }
            SystemType::UpdateTimedLife => {
                self.data.execute("UPDATE entities SET timed_life = timed_life - ?;", vec![delta]);
            }
            SystemType::Shoot => {
                self.data.execute("UPDATE entities SET shooter_cooldown = shooter_cooldown - ?;", vec![delta]);
            }
            SystemType::Acceleration => {
                self.data.execute("UPDATE entities SET velocity_x = velocity_x + acceleration_x * ?;", vec![delta]);
                self.data.execute("UPDATE entities SET velocity_y = velocity_y + acceleration_y * ?;", vec![delta]);
            }
            SystemType::MapEdge => {}
            SystemType::DeleteExpired => {}
            SystemType::PaintNearest => {}
        }
    }

    fn get_name(&self) -> String {
        "BrainDatabase".to_string()
    }
}