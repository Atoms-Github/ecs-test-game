use crate::brains::com::ExportEntity;
use crate::brains::sql_brains::brain_sql::CommandPlanSql;
use crate::brains::sql_interfaces::{SqlInterface, SqlStatement};
use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::{color_from_team, FromTeam};
use crate::{Point, MAP_SIZE};
use duckdb::ffi::system;
use ggez::graphics::Color;
use std::process::id;

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
                    SqlStatement::new("UPDATE entities SET position_x = position_x + velocity_x * ? WHERE velocity_x IS NOT NULL;", vec![delta]),
                    SqlStatement::new("UPDATE entities SET position_y = position_y + velocity_y * ? WHERE velocity_y IS NOT NULL;", vec![delta]),
                ]
            }
            SystemType::UpdateTimedLife => {
                vec![SqlStatement::new(
                    "UPDATE entities SET timed_life = timed_life - ?;",
                    vec![delta],
                )]
            }
            SystemType::Shoot => {
                let cooldown_update = SqlStatement::new(
                    "UPDATE entities SET shooter_cooldown = shooter_cooldown - ? WHERE shooter_cooldown IS NOT NULL;",
                    vec![delta],
                );
                let shooters_temp_table = SqlStatement::new(
                    "CREATE TEMPORARY TABLE shooters AS SELECT id, position_x, position_y, shooter_cooldown, team, universe_id FROM entities WHERE shooter_cooldown IS NOT NULL AND shooter_cooldown < 0.0;",
                    vec![],
                );
                // pair each shooter with every other entity in the universe
                // Same universe, different team, and not self.
                // Create a table with columns: shooter_id, target_id, distance



                let closest_targets_temp_table = SqlStatement::new(
                    "CREATE TEMPORARY TABLE closest_targets_temp AS
                    SELECT shooters.id AS shooter_id, entities.id AS target_id, entities.position_x AS target_x, entities.position_y AS target_y,
                    (shooters.position_x - entities.position_x) * (shooters.position_x - entities.position_x) + (shooters.position_y - entities.position_y) * (shooters.position_y - entities.position_y) AS distance
                    FROM shooters
                    JOIN entities ON entities.universe_id = shooters.universe_id
                    WHERE entities.team != shooters.team
                    AND entities.id != shooters.id;",
                    vec![],
                );
                let closest = SqlStatement::new(
                    "CREATE TEMPORARY TABLE closest_targets AS SELECT * \
                     FROM closest_targets_temp WHERE distance = (SELECT MIN(distance) FROM closest_targets_temp i WHERE i.shooter_id = closest_targets_temp.shooter_id);",

                    vec![]
                );




                // Insert a projectile for each item in the temp table
                let insert_projectiles = SqlStatement::new(
                    "INSERT INTO entities (position_x, position_y, velocity_x, velocity_y, universe_id, blue, timed_life)
                    SELECT shooters.position_x, shooters.position_y, closest_targets.target_x - shooters.position_x, closest_targets.target_y - shooters.position_y, shooters.universe_id, 0.3, 2.0
                    FROM closest_targets
                    JOIN shooters ON shooters.id = closest_targets.shooter_id;",
                    vec![],
                );

                let reset_cooldown_for_shooters = SqlStatement::new(
                    "UPDATE entities SET shooter_cooldown = 0.5 WHERE id IN (SELECT id FROM shooters);",
                    vec![],
                );
                vec![
                    cooldown_update,
                    shooters_temp_table,
                    closest_targets_temp_table,
                    closest,
                    insert_projectiles,
                    reset_cooldown_for_shooters,

                    // And drop the temp tables
                    SqlStatement::new("DROP TABLE shooters;", vec![]),
                    SqlStatement::new("DROP TABLE closest_targets;", vec![]),
                    SqlStatement::new("DROP TABLE closest_targets_temp;", vec![]),
                ]
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
                        "UPDATE entities SET position_x = position_x - ? WHERE position_x > ?;",
                        vec![MAP_SIZE, MAP_SIZE],
                    ),
                    SqlStatement::new(
                        "UPDATE entities SET position_x = position_x + ? WHERE position_x < 0;",
                        vec![MAP_SIZE],
                    ),
                    SqlStatement::new(
                        "UPDATE entities SET position_y = position_y - ? WHERE position_y > ?;",
                        vec![MAP_SIZE, MAP_SIZE],
                    ),
                    SqlStatement::new(
                        "UPDATE entities SET position_y = position_y + ? WHERE position_y < 0;",
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
            "INSERT INTO entities (position_x, position_y, velocity_x, velocity_y, team, universe_id, blue, shooter_cooldown) VALUES (?, ?, ?, ?, ?, ?, ?, ?);",
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
                "INSERT INTO entities (position_x, position_y, velocity_x, velocity_y, blue, universe_id) VALUES (?, ?, ?, ?, ?, 0);",
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
                "INSERT INTO entities (position_x, position_y, blue, universe_id) VALUES (?, ?, ?, 0);",
                vec![position.x, position.y, blue],
            )
        };
    }
    fn get_ents_xyc(&mut self, universe_id: usize) -> SqlStatement {
        return SqlStatement::new(
            "SELECT position_x, position_y, blue FROM entities WHERE universe_id = ?;",
            vec![universe_id as f32],
        );
    }

    fn init_systems(&mut self, systems: &Vec<SystemType>) -> Vec<SqlStatement> {
        // Entities should have an auto incrementing id
        return vec![SqlStatement::new(
            "CREATE TABLE entities (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            position_x REAL,
            position_y REAL,
            velocity_x REAL,
            velocity_y REAL,
            acceleration_x REAL,
            acceleration_y REAL,
            blue REAL,
            team INTEGER,
            universe_id INTEGER,
            shooter_cooldown REAL,
            timed_life REAL
        );",
            vec![],
        )];
    }
}
