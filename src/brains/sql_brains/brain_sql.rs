use crate::brains::com::ExportEntity;
use crate::brains::sql_interfaces::{SqlInterface, SqlStatement};
use crate::brains::{Brain, SystemType};
use crate::simulation_settings::SimSettings;
use crate::ui::ui_settings::GuiSettings;
use crate::Point;
use duckdb::params;
use ggez::graphics::Color;

pub struct BrainSql<C, I> {
    command_plan: C,
    database: I,
}
pub trait CommandPlanSql {
    fn systems<I: SqlInterface>(
        &mut self,
        sys: &SystemType,
        delta: f32,
        settings: &SimSettings,
    ) -> Vec<SqlStatement>;
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    ) -> SqlStatement;
    fn add_entity(&mut self, position: Point, velocity: Option<Point>, blue: f32) -> SqlStatement;
    fn add_entity_blob(&mut self, position: Point, blob: Vec<u8>, blue: f32) -> SqlStatement;
    fn get_ents_xyc(&mut self, universe_id: usize) -> SqlStatement;
    fn init_systems<I: SqlInterface>(&mut self, systems: &Vec<SystemType>) -> Vec<SqlStatement>;
}
impl<C, D> BrainSql<C, D> {
    pub fn new(c: C, d: D) -> Self {
        Self {
            command_plan: c,
            database: d,
        }
    }
}

impl<C: CommandPlanSql, I: SqlInterface> Brain for BrainSql<C, I> {
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
        self.database.execute_batch(vec![command]);
    }

    fn add_entity(&mut self, position: Point, velocity: Option<Point>, blue: f32) {
        let command = self.command_plan.add_entity(position, velocity, blue);
        self.database.execute_batch(vec![command]);
    }

    fn add_entity_blob(&mut self, position: Point, blob: Vec<u8>, blue: f32) {
        let command = self.command_plan.add_entity_blob(position, blob, blue);
        self.database.execute_batch(vec![command]);
    }

    fn get_entities(&mut self, universe_id: usize) -> Vec<ExportEntity> {
        let command = self.command_plan.get_ents_xyc(universe_id);
        let entities = self.database.get_entities(command);
        return entities;
    }
    fn init(&mut self, systems: &Vec<SystemType>) {
        let commands = self.command_plan.init_systems::<I>(systems);
        self.database.execute_batch(commands);
    }

    fn tick_systems(&mut self, delta: f32, settings: &SimSettings, systems: &Vec<SystemType>) {
        let mut commands = vec![];
        for sys in systems {
            let mut sys_commands = self.command_plan.systems::<I>(sys, delta, settings);
            commands.extend(sys_commands);
        }
        self.database.execute_batch(commands);
    }
    fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &SimSettings) {
        let commands = self.command_plan.systems::<I>(system, delta, settings);
        for command in commands {
            // Deliberately not doing batch execution here, to compare performance
            self.database.execute_single(command);
        }
    }
    fn get_name(&self) -> String {
        return "BrainSql".to_string();
    }
}
