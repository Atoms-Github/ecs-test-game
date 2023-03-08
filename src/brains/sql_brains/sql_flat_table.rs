use std::process::id;

use duckdb::ffi::system;
use ggez::graphics::Color;

use crate::brains::com::ExportEntity;
use crate::brains::sql_brains::brain_sql::CommandPlanSql;
use crate::brains::sql_interfaces::{InterfaceType, SqlArgument, SqlInterface, SqlStatement};
use crate::brains::{Brain, SystemType};
use crate::simulation_settings::BrainType::SqlIte;
use crate::simulation_settings::{BrainType, SimSettings};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::{color_from_team, FromTeam};
use crate::{Point, MAP_SIZE, PROJECTILE_LIFETIME, SHOOT_SPEED};

pub struct BrainSqlFlatTable {}
impl BrainSqlFlatTable {
	pub fn new() -> Self {
		Self {}
	}
}
impl CommandPlanSql for BrainSqlFlatTable {
	fn systems<I: SqlInterface>(
		&mut self,
		sys: &SystemType,
		delta: f32,
		settings: &SimSettings,
	) -> Vec<SqlStatement> {
		let mut statements: Vec<SqlStatement> = match sys {
			SystemType::Velocity => {
				vec![
                    SqlStatement::new_f32("UPDATE entities SET position_x = position_x + velocity_x * ? WHERE velocity_x IS NOT NULL;", vec![delta]),
                    SqlStatement::new_f32("UPDATE entities SET position_y = position_y + velocity_y * ? WHERE velocity_y IS NOT NULL;", vec![delta]),
                ]
			}
			SystemType::UpdateTimedLife => {
				vec![SqlStatement::new_f32(
					"UPDATE entities SET timed_life = timed_life - ?;",
					vec![delta],
				)]
			}
			SystemType::Shoot => {
				let cooldown_update = SqlStatement::new_f32(
                    "UPDATE entities SET shooter_cooldown = shooter_cooldown - ? WHERE shooter_cooldown IS NOT NULL;",
                    vec![delta],
                );
				let shooters_temp_table = SqlStatement::new_f32(
                    "CREATE TEMPORARY TABLE shooters AS SELECT id, position_x, position_y, shooter_cooldown, team, universe_id FROM entities WHERE shooter_cooldown IS NOT NULL AND shooter_cooldown < 0.0;",
                    vec![],
                );
				// pair each shooter with every other entity in the universe
				// Same universe, different team, and not self.
				// Create a table with columns: shooter_id, target_id, distance

				let closest_targets_temp_table = SqlStatement::new_f32(
                    "CREATE TEMPORARY TABLE closest_targets_temp AS
                    SELECT shooters.id AS shooter_id, entities.id AS target_id, entities.position_x AS target_x, entities.position_y AS target_y,
                    (shooters.position_x - entities.position_x) * (shooters.position_x - entities.position_x) + (shooters.position_y - entities.position_y) * (shooters.position_y - entities.position_y) AS distance
                    FROM shooters
                    JOIN entities ON entities.universe_id = shooters.universe_id
                    WHERE entities.team != shooters.team
                    AND entities.id != shooters.id;",
                    vec![]
                );
				let shoot_distance = settings.rts_range * settings.rts_range;
				let closest = SqlStatement::new_f32(
                    "CREATE TEMPORARY TABLE closest_targets AS SELECT * \
                     FROM closest_targets_temp WHERE distance = (SELECT MIN(distance) FROM \
                      closest_targets_temp i WHERE i.shooter_id = closest_targets_temp.shooter_id) AND distance <= ?  ;",

                    vec![shoot_distance],
                );

				// Insert a projectile for each item in the temp table
				let insert_projectiles = SqlStatement::new_f32(
                    "INSERT INTO entities (position_x, position_y, velocity_x, velocity_y, universe_id, blue, timed_life)
                    SELECT shooters.position_x, shooters.position_y, closest_targets.target_x - shooters.position_x, closest_targets.target_y - shooters.position_y, shooters.universe_id, 0.3, ?
                    FROM closest_targets
                    JOIN shooters ON shooters.id = closest_targets.shooter_id;",
                    vec![PROJECTILE_LIFETIME],
                );
				// Normalize velocity of projectiles
				let normalize_projectiles = SqlStatement::new_f32(
                    "UPDATE entities SET velocity_x = velocity_x / SQRT(velocity_x * velocity_x + velocity_y * velocity_y), velocity_y = velocity_y / SQRT(velocity_x * velocity_x + velocity_y * velocity_y) WHERE blue = 0.3;",
                    vec![],
                );

				let reset_cooldown_for_shooters = SqlStatement::new_f32(
					"UPDATE entities SET shooter_cooldown = ? WHERE id IN (SELECT id FROM shooters);",
					vec![SHOOT_SPEED],
				);
				vec![
					cooldown_update,
					shooters_temp_table,
					closest_targets_temp_table,
					closest,
					insert_projectiles,
					reset_cooldown_for_shooters,
					// And drop the temp tables
					SqlStatement::new_f32("DROP TABLE shooters;", vec![]),
					SqlStatement::new_f32("DROP TABLE closest_targets;", vec![]),
					SqlStatement::new_f32("DROP TABLE closest_targets_temp;", vec![]),
				]
			}
			SystemType::Acceleration => {
				vec![
                    SqlStatement::new_f32("UPDATE entities SET velocity_x = velocity_x + acceleration_x * ? WHERE acceleration_x IS NOT NULL;", vec![delta]),
                    SqlStatement::new_f32("UPDATE entities SET velocity_y = velocity_y + acceleration_y * ? WHERE acceleration_y IS NOT NULL;", vec![delta]),
                ]
			}
			SystemType::MapEdge => {
				vec![
					SqlStatement::new_f32(
						"UPDATE entities SET position_x = position_x - ? WHERE position_x > ?;",
						vec![MAP_SIZE, MAP_SIZE],
					),
					SqlStatement::new_f32(
						"UPDATE entities SET position_x = position_x + ? WHERE position_x < 0;",
						vec![MAP_SIZE],
					),
					SqlStatement::new_f32(
						"UPDATE entities SET position_y = position_y - ? WHERE position_y > ?;",
						vec![MAP_SIZE, MAP_SIZE],
					),
					SqlStatement::new_f32(
						"UPDATE entities SET position_y = position_y + ? WHERE position_y < 0;",
						vec![MAP_SIZE],
					),
				]
			}
			SystemType::DeleteExpired => {
				vec![SqlStatement::new_f32(
					"DELETE FROM entities WHERE timed_life < 0;",
					vec![],
				)]
			}
			SystemType::PaintNearest => {
				let blend_speed = settings.paint_speed + 1.0;
				match I::get_type() {
					InterfaceType::Sqlite => {
						let update_blue = SqlStatement::new_f32(
                            "WITH cte AS (
  SELECT
    e2.blue as closest_blue,
    e1.id,
    ((e1.position_x - e2.position_x) * (e1.position_x - e2.position_x) + (e1.position_y - e2.position_y) * (e1.position_y - e2.position_y)) AS distance
  FROM entities AS e1
  JOIN entities AS e2
  ON e1.universe_id = e2.universe_id
  AND e1.id != e2.id
)
UPDATE entities AS e1
SET blue = e1.blue + (SELECT closest_blue FROM (
  SELECT
    closest_blue,
    id,
    distance,
    ROW_NUMBER() OVER (PARTITION BY id ORDER BY distance) AS rn
  FROM cte
) WHERE rn = 1 and id = e1.id) / 10000.0",
                            vec![],
                        );
						let mod_blue = SqlStatement::new_f32(
							"UPDATE entities SET blue = blue - 1.0 where blue > 1.0;",
							vec![],
						);
						vec![update_blue, mod_blue]
					}
					_ => {
						let update_blue = SqlStatement::new_f32(
                            "UPDATE entities e1
SET blue = e1.blue +
  (SELECT e2.blue
   FROM entities e2
   WHERE e2.universe_id = e1.universe_id
   AND e2.id != e1.id
   ORDER BY (e1.position_x - e2.position_x) * (e1.position_x - e2.position_x) + (e1.position_y - e2.position_y) * (e1.position_y - e2.position_y)
   LIMIT 1) / 10000",
                            vec![],
                        );
						let mod_blue = SqlStatement::new_f32(
							"UPDATE entities SET blue = blue - 1.0 where blue > 1.0;",
							vec![],
						);
						vec![update_blue, mod_blue]
					}
				}
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
		return SqlStatement::new_f32(
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
			SqlStatement::new_f32(
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
			SqlStatement::new_f32(
				"INSERT INTO entities (position_x, position_y, blue, universe_id) VALUES (?, ?, ?, 0);",
				vec![position.x, position.y, blue],
			)
		};
	}

	fn add_entity_blob(&mut self, position: Point, blob: Vec<u8>, blue: f32) -> SqlStatement {
		return SqlStatement::new(
			"INSERT INTO entities (position_x, position_y, blob, universe_id, blue) VALUES (?, ?, ?, 0, ?);",
			vec![
				SqlArgument::Float(position.x),
				SqlArgument::Float(position.y),
				SqlArgument::Blob(blob),
				SqlArgument::Float(blue),
			],
		);
	}

	fn get_ents_xyc(&mut self, universe_id: usize) -> SqlStatement {
		return SqlStatement::new_f32(
			"SELECT position_x, position_y, blue FROM entities WHERE universe_id = ?;",
			vec![universe_id as f32],
		);
	}

	fn get_image(&mut self, entity_id: u64) -> SqlStatement {
		return SqlStatement::new_f32(
			"SELECT blob FROM entities WHERE id = ?;",
			vec![entity_id as f32],
		);
	}

	fn init_systems<I: SqlInterface>(&mut self, systems: &Vec<SystemType>) -> Vec<SqlStatement> {
		let mut systems = vec![];
		match I::get_type() {
			InterfaceType::DuckDB => {
				systems.append(&mut vec![
					SqlStatement::new_f32(
						format!(
							"CREATE TABLE entities (
            id INTEGER PRIMARY KEY NOT NULL,
            position_x REAL,
            position_y REAL,
            velocity_x REAL,
            velocity_y REAL,
            acceleration_x REAL,
            acceleration_y REAL,
            blob BLOB,
            blue REAL,
            team INTEGER,
            universe_id INTEGER,
            shooter_cooldown REAL,
            timed_life REAL
        );",
						)
						.as_str(),
						vec![],
					),
					SqlStatement::new_f32("CREATE SEQUENCE entities_id_seq;", vec![]),
					SqlStatement::new_f32(
						"ALTER TABLE entities ALTER COLUMN id SET DEFAULT nextval('entities_id_seq');",
						vec![],
					),
				]);
			}
			_ => {
				systems.append(&mut vec![SqlStatement::new_f32(
					format!(
						"CREATE TABLE entities (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            position_x REAL,
            position_y REAL,
            velocity_x REAL,
            velocity_y REAL,
            acceleration_x REAL,
            acceleration_y REAL,
            blob BLOB,
            blue REAL,
            team INTEGER,
            universe_id INTEGER,
            shooter_cooldown REAL,
            timed_life REAL
        );",
					)
					.as_str(),
					vec![],
				)]);
			}
		}
		systems.append(&mut vec![
			SqlStatement::new_f32("CREATE INDEX idx_team ON entities (team);", vec![]),
			SqlStatement::new_f32("CREATE INDEX idx_id ON entities (id);", vec![]),
			SqlStatement::new_f32("CREATE INDEX idx_universe ON entities (universe_id);", vec![]),
		]);

		return systems;
	}
}
