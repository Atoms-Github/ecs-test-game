use crate::brains::com::ExportEntity;
use crate::brains::sql_brains::brain_sql::CommandPlanSql;
use crate::brains::sql_interfaces::{SqlInterface, SqlStatement};
use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::{color_from_team, FromTeam};
use crate::{Point, MAP_SIZE};
use duckdb::ffi::system;
use ggez::graphics::Color;

pub struct BrainSqlFlatTable {}
impl BrainSqlFlatTable {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandPlanSql for BrainSqlFlatTable {
    fn systems(
        &mut self,
        sys: &SystemType,
        delta: f32,
        settings: &GuiSettings,
    ) -> Vec<SqlStatement> {
        let mut statements: Vec<SqlStatement> = match sys {
            SystemType::Velocity => {
                vec![
                    SqlStatement::new("UPDATE entities SET pos_x = pos_x + velocity_x * ? WHERE velocity_x IS NOT NULL;", vec![delta]),
                    SqlStatement::new("UPDATE entities SET pos_y = pos_y + velocity_y * ? WHERE velocity_y IS NOT NULL;", vec![delta]),
                ]
            }
            SystemType::UpdateTimedLife => {
                vec![SqlStatement::new(
                    "UPDATE entities SET timed_life = timed_life - ?;",
                    vec![delta],
                )]
            }
            SystemType::Shoot => {
                vec![SqlStatement::new(
                    "UPDATE entities SET shooter_cooldown = shooter_cooldown - ?;",
                    vec![delta],
                )]
            }
            SystemType::Acceleration => {
                vec![
                    SqlStatement::new("UPDATE entities SET velocity_x = velocity_x + acceleration_x * ? WHERE acceleration_x IS NOT NULL;", vec![delta]),
                    SqlStatement::new("UPDATE entities SET velocity_y = velocity_y + acceleration_y * ? WHERE acceleration_y IS NOT NULL;", vec![delta]),
                ]
            }
            SystemType::MapEdge => {
                vec![
                    SqlStatement::new(
                        "UPDATE entities SET pos_x = pos_x - ? WHERE pos_x > ?;",
                        vec![MAP_SIZE, MAP_SIZE],
                    ),
                    SqlStatement::new(
                        "UPDATE entities SET pos_x = pos_x + ? WHERE pos_x < 0;",
                        vec![MAP_SIZE],
                    ),
                    SqlStatement::new(
                        "UPDATE entities SET pos_y = pos_y - ? WHERE pos_y > ?;",
                        vec![MAP_SIZE, MAP_SIZE],
                    ),
                    SqlStatement::new(
                        "UPDATE entities SET pos_y = pos_y + ? WHERE pos_y < 0;",
                        vec![MAP_SIZE],
                    ),
                ]
            }
            SystemType::DeleteExpired => {
                vec![SqlStatement::new(
                    "DELETE FROM entities WHERE timed_life < 0;",
                    vec![],
                )]
            }
            SystemType::PaintNearest => {
                vec![]
            }
        };
        return statements;
    }
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    ) -> SqlStatement {
        let blue = color_from_team(team);
        return SqlStatement::new(
            "INSERT INTO entities (position_x, position_y, velocity_x, velocity_y, team, universe_id, blue, shooter_cooldown) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            vec![
                position.x,
                position.y,
                velocity.x,
                velocity.y,
                team as f32,
                universe_id as f32,
                blue,
                0.0,
            ],
        );
    }

    fn add_entity(&mut self, position: Point, velocity: Option<Point>, blue: f32) -> SqlStatement {
        return if let Some(velocity) = velocity {
            SqlStatement::new(
                "INSERT INTO entities (position_x, position_y, velocity_x, velocity_y, blue) VALUES (?, ?, ?, ?, ?)",
                vec![
                    position.x,
                    position.y,
                    velocity.x,
                    velocity.y,
                    blue,
                ],
            )
        } else {
            SqlStatement::new(
                "INSERT INTO entities (position_x, position_y, blue) VALUES (?, ?, ?)",
                vec![position.x, position.y, blue],
            )
        };
    }

    fn get_ents_xyc(&mut self, universe_id: usize) -> SqlStatement {
        let command = "SELECT pos_x, pos_y, color_r FROM entities";
        return SqlStatement::new(command, vec![]);
    }

    fn init_systems(&mut self, systems: &Vec<SystemType>) -> Vec<SqlStatement> {
        return vec![SqlStatement::new(
            "CREATE TABLE entities (
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
        )",
            vec![],
        )];
    }
}
