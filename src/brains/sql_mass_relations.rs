use ggez::graphics::Color;
use crate::brains::{Brain, SystemType};
use crate::brains::sql_brains::SqlBrain;
use crate::{MAP_SIZE, Point};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::FromTeam;

pub struct BrainSqlMassRelations<T> {
    database: T,
    simultaneous: bool,

}


impl<T : SqlBrain> BrainSqlMassRelations<T>{
    pub fn new(simultaneous: bool) -> Self {
        Self {
            database: T::new(),
            simultaneous
        }
    }
}

impl<T : SqlBrain> Brain for BrainSqlMassRelations<T> {
    fn add_entity_unit(&mut self, position: Point, velocity: Point, team: usize, universe_id: usize) {
        let red = Color::from_team(team).r;
        self.database.execute("INSERT INTO entities (pos_x, pos_y, velocity_x, velocity_y, team, universe_id, color_r) VALUES (?, ?, ?, ?, ?, ?, ?)",
                              vec![position.x, position.y, velocity.x, velocity.y, team as f32, universe_id as f32, red as f32]);
    }

    fn add_entity_vel_dot(&mut self, position: Point, velocity: Point) {
    }

    fn add_entity_positional_dummy(&mut self, position: Point, color: Color) {
    }

    fn get_entities(&mut self, universe_id: usize) -> Vec<(Point, Color)> {
        return self.database.get_entities(universe_id, "SELECT pos_x, pos_y, color_r FROM entities").iter().map(|e| (e.pos, e.color)).collect();
    }

    fn init_systems(&mut self, systems: &Vec<SystemType>) {
        self.database.execute("CREATE TABLE entities (
            pos_x REAL,
            pos_y REAL,
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
        self.simultaneous
    }

    fn tick_systems(&mut self, delta: f32, settings: &GuiSettings) {
        // TODO:
    }
    fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &GuiSettings) {
        match system{
            SystemType::Velocity => {
                self.database.execute("UPDATE entities SET pos_x = pos_x + velocity_x * ? WHERE velocity_x IS NOT NULL;", vec![delta]);
                self.database.execute("UPDATE entities SET pos_y = pos_y + velocity_y * ? WHERE velocity_y IS NOT NULL;", vec![delta]);
                // Update pos for every entity that has velocity
            }
            SystemType::UpdateTimedLife => {
                self.database.execute("UPDATE entities SET timed_life = timed_life - ?;", vec![delta]);
            }
            SystemType::Shoot => {
                self.database.execute("UPDATE entities SET shooter_cooldown = shooter_cooldown - ?;", vec![delta]);
            }
            SystemType::Acceleration => {
                self.database.execute("UPDATE entities SET velocity_x = velocity_x + acceleration_x * ? WHERE acceleration_x IS NOT NULL;", vec![delta]);
                self.database.execute("UPDATE entities SET velocity_y = velocity_y + acceleration_y * ? WHERE acceleration_y IS NOT NULL;", vec![delta]);
            }
            SystemType::MapEdge => {
               // If the entity is outside the map, move it to the other side
                self.database.execute("UPDATE entities SET pos_x = pos_x - ? WHERE pos_x > ?;", vec![MAP_SIZE, MAP_SIZE]);
                self.database.execute("UPDATE entities SET pos_x = pos_x + ? WHERE pos_x < 0;", vec![MAP_SIZE]);
                self.database.execute("UPDATE entities SET pos_y = pos_y - ? WHERE pos_y > ?;", vec![MAP_SIZE, MAP_SIZE]);
                self.database.execute("UPDATE entities SET pos_y = pos_y + ? WHERE pos_y < 0;", vec![MAP_SIZE]);

            }

            SystemType::DeleteExpired => {
                self.database.execute("DELETE FROM entities WHERE timed_life < 0;", vec![]);
            }
            SystemType::PaintNearest => {

            }
        }
    }

    fn get_name(&self) -> String {
        "BrainDatabase".to_string()
    }
}
