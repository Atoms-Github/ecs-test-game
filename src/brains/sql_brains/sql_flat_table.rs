use crate::brains::com::ExportEntity;
use crate::brains::sql_brains::sql_brain::SqlCommandPlan;
use crate::brains::sql_interfaces::{SqlInterface, SqlStatement};
use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::FromTeam;
use crate::{Point, MAP_SIZE};
use ggez::graphics::Color;

pub struct BrainSqlFlatTable {}

impl SqlCommandPlan for BrainSqlFlatTable {
    fn systems(
        &mut self,
        sys: &SystemType,
        delta: f32,
        settings: &GuiSettings,
    ) -> Vec<SqlStatement> {
    }

    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    ) -> SqlStatement {
        todo!()
    }

    fn add_entity(
        &mut self,
        position: Point,
        velocity: Option<Point>,
        color: Color,
    ) -> SqlStatement {
        todo!()
    }

    fn get_ents_xyc(&mut self, universe_id: usize) -> SqlStatement {
        todo!()
    }

    fn init_systems(&mut self, systems: &Vec<SystemType>) -> Vec<SqlStatement> {
        todo!()
    }
}
impl<T: SqlInterface> Brain for BrainSqlFlatTable<T> {
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    ) {
        let red = Color::from_team(team).r;
        self.database.execute();
    }

    fn add_entity(&mut self, position: Point, velocity: Option<Point>, color: Color) {}

    fn get_entities(&mut self, universe_id: usize) -> Vec<ExportEntity> {
        return self
            .database
            .get_entities()
            .iter()
            .map(|e| (e.pos, e.color))
            .collect();
    }

    fn init(&mut self, systems: &Vec<SystemType>) {
        self.database.execute();
    }

    fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &GuiSettings) {
        match system {
            SystemType::Velocity => {
                self.database.execute();
                self.database.execute();
                // Update pos for every entity that has velocity
            }
            SystemType::UpdateTimedLife => {
                self.database.execute();
            }
            SystemType::Shoot => {
                self.database.execute();
            }
            SystemType::Acceleration => {
                self.database.execute();
                self.database.execute();
            }
            SystemType::MapEdge => {
                // If the entity is outside the map, move it to the other side
                self.database.execute();
                self.database.execute();
                self.database.execute();
                self.database.execute();
            }

            SystemType::DeleteExpired => {
                self.database.execute();
            }
            SystemType::PaintNearest => {}
        }
    }

    fn get_name(&self) -> String {
        "BrainDatabase".to_string()
    }
}
