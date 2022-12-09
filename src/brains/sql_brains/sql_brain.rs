use crate::brains::com::ExportEntity;
use crate::brains::sql_interfaces::{SqlInterface, SqlStatement};
use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;
use crate::Point;
use duckdb::params;
use ggez::graphics::Color;

pub struct BrainSql<C, D> {
    command_plan: C,
    database: D,
}
pub trait SqlCommandPlan {
    fn systems(
        &mut self,
        sys: &SystemType,
        delta: f32,
        settings: &GuiSettings,
    ) -> Vec<SqlStatement>;
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    ) -> SqlStatement;
    fn add_entity(
        &mut self,
        position: Point,
        velocity: Option<Point>,
        color: Color,
    ) -> SqlStatement;
    fn get_ents_xyc(&mut self, universe_id: usize) -> SqlStatement;
    fn init_systems(&mut self, systems: &Vec<SystemType>) -> Vec<SqlStatement>;
}
impl<C, D> BrainSql<C, D> {
    fn new(c: C, d: D) -> Self {
        Self {
            command_plan: c,
            database: d,
        }
    }
}

impl<C: SqlCommandPlan, D: SqlInterface> Brain for BrainSql<C, D> {
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    ) {
        let command = self
            .command_plan
            .add_entity_unit(position, velocity, team, universe_id);
        self.database.execute(command);
    }

    fn add_entity(&mut self, position: Point, velocity: Option<Point>, color: Color) {
        let command = self.command_plan.add_entity(position, velocity, color);
        self.database.execute(command);
    }

    fn get_entities(&mut self, universe_id: usize) -> Vec<ExportEntity> {
        let command = self.command_plan.get_ents_xyc(universe_id);
        let entities = self.database.get_entities(command);
        return entities;
    }

    fn init(&mut self, systems: &Vec<SystemType>) {
        let commands = self.command_plan.init_systems(systems);
        for command in commands {
            self.database.execute(command);
        }
    }

    fn tick_systems(&mut self, delta: f32, settings: &GuiSettings, systems: &Vec<SystemType>) {
        let mut big_command = String::new();
        let mut params = Vec::new();
        for system in systems {
            let commands = self.command_plan.systems(system, delta, settings);
            for command in commands {
                big_command.push_str(&command.sql);
                big_command.push(';');
                params.extend(command.params);
            }
        }
        self.database.execute(SqlStatement {
            statement: big_command,
            params,
        });
    }
    fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &GuiSettings) {
        let commands = self.command_plan.systems(system, delta, settings);
        for command in commands {
            self.database.execute(command);
        }
    }
    fn get_name(&self) -> String {
        return "BrainSql".to_string();
    }
}
