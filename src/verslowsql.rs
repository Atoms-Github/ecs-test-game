use crate::rts::*;

use crate::rts::STARTING_VELOCITY;
use crate::rts::WORLD_SIZE;
use ggez::graphics::Color;
use glam::*;
use legion::systems::CommandBuffer;
use legion::*;
use rand::Rng;
use sqlite::*;

pub struct VerySlowSQL {
    connection: Connection,
    current_ent_id: i64,
}

impl VerySlowSQL {
    pub fn new() -> VerySlowSQL {
        let connection = open(":memory:").unwrap();
        // Create a table with a single column of entity IDs, which is the primary ID
        connection
            .execute(
                "CREATE TABLE entities (`id` INT NOT NULL,
  PRIMARY KEY (`id`))",
            )
            .unwrap();

        // Create a table of position components:
        connection
            .execute(
                "CREATE TABLE position (
            id INTEGER PRIMARY KEY,
            position_x REAL,
            position_y REAL,
            FOREIGN KEY (id) REFERENCES entities(id)
        )",
            )
            .unwrap();
        // Create a table of velocity components:
        connection
            .execute(
                "CREATE TABLE velocity (
            id INTEGER PRIMARY KEY,
            velocity_x REAL,
            velocity_y REAL,
            FOREIGN KEY (id) REFERENCES entities(id)
        )",
            )
            .unwrap();
        // Create a table of team components:
        connection
            .execute(
                "CREATE TABLE team (
            id INTEGER PRIMARY KEY,
            team INTEGER,
            FOREIGN KEY (id) REFERENCES entities(id)
        )",
            )
            .unwrap();
        // Create a table of universe components:
        connection
            .execute(
                "CREATE TABLE universe (
            id INTEGER PRIMARY KEY,
            universe_id INTEGER,
            FOREIGN KEY (id) REFERENCES entities(id)
        )",
            )
            .unwrap();
        // Create a table of shooter components:
        connection
            .execute(
                "CREATE TABLE shooter (
            id INTEGER PRIMARY KEY,
            shooter_cooldown REAL,
            FOREIGN KEY (id) REFERENCES entities(id)
        )",
            )
            .unwrap();
        // Create a table of timed_life components:
        connection
            .execute(
                "CREATE TABLE timed_life (
            id INTEGER PRIMARY KEY,
            timed_life REAL,
            FOREIGN KEY (id) REFERENCES entities(id)
        )",
            )
            .unwrap();
        VerySlowSQL {
            connection,
            current_ent_id: 0,
        }
    }
}
impl GameImplementation for VerySlowSQL {
    fn update(&mut self, dt: f32, settings: &GuiSettings) {
        let mut resources = Resources::default();
        // Start transaction:
        self.connection.execute("BEGIN").unwrap();

        // Update position components by joining velocity components on entity IDs:
        self.connection
            .prepare(
                "UPDATE position
            SET position_x = position_x + velocity_x * ?,
                position_y = position_y + velocity_y * ?
            FROM velocity
            WHERE position.id = velocity.id;",
            )
            .unwrap()
            .bind(1, dt as f64)
            .unwrap()
            .bind(2, dt as f64)
            .unwrap()
            .next()
            .unwrap();
        // Update timed life components:
        self.connection
            .prepare(
                "UPDATE timed_life
            SET timed_life = timed_life - ?;",
            )
            .unwrap()
            .bind(1, dt as f64)
            .unwrap()
            .next()
            .unwrap();
        // Update shooter component:
        self.connection
            .prepare(
                "UPDATE shooter
            SET shooter_cooldown = shooter_cooldown - ?;",
            )
            .unwrap()
            .bind(1, dt as f64)
            .unwrap()
            .next()
            .unwrap();

        self.connection.execute("COMMIT").unwrap();
    }

    fn get_unit_positions(&self, universe_id: usize) -> Vec<(Vec2, Color)> {
        let mut positions_and_teams = Vec::new();
        // Get the positions and teams of all entities in the universe:
        let mut statement = self
            .connection
            .prepare("SELECT position_x, position_y, team FROM ((position INNER JOIN team ON position.id = team.id) as myfirst INNER JOIN universe ON universe.id = myfirst.id) WHERE universe.universe_id = ?")
            .unwrap()
            .bind(1, universe_id as i64)
            .unwrap();

        while let Ok(State::Row) = statement.next() {
            let position_x: f64 = statement.read(0).unwrap();
            let position_y: f64 = statement.read(1).unwrap();
            let team: i64 = statement.read(2).unwrap();

            let color = match team {
                0 => Color::from_rgb(255, 0, 0),
                1 => Color::from_rgb(0, 255, 0),
                2 => Color::from_rgb(0, 0, 255),
                _ => Color::from_rgb(255, 255, 255),
            };

            positions_and_teams.push((vec2(position_x as f32, position_y as f32), color));
        }
        positions_and_teams
    }

    fn load_universe(&mut self, universe_id: usize, entity_count: i64) {
        let mut rng = rand::thread_rng();
        for index in 0..entity_count {
            let i = self.current_ent_id;
            self.current_ent_id += 1;
            let pos = Vec2::new(
                rng.gen_range(0.0..WORLD_SIZE),
                rng.gen_range(0.0..WORLD_SIZE),
            );
            let vel = Vec2::new(
                rng.gen_range(0.0..STARTING_VELOCITY),
                rng.gen_range(0.0..STARTING_VELOCITY),
            );
            let team = rng.gen_range(0..=2);

            // Insert a new entity with id i into the database using format!
            self.connection
                .execute(&format!("INSERT INTO entities (id) VALUES ({})", i))
                .unwrap();

            self.connection
                .prepare(
                    "INSERT INTO position (id, position_x, position_y)
                VALUES (?, ?, ?)",
                )
                .unwrap()
                .bind(1, i)
                .unwrap()
                .bind(2, pos.x as f64)
                .unwrap()
                .bind(3, pos.y as f64)
                .unwrap()
                .next()
                .unwrap();
            // Insert velocity component:
            self.connection
                .prepare(
                    "INSERT INTO velocity (id, velocity_x, velocity_y)
                VALUES (?, ?, ?)",
                )
                .unwrap()
                .bind(1, i)
                .unwrap()
                .bind(2, vel.x as f64)
                .unwrap()
                .bind(3, vel.y as f64)
                .unwrap()
                .next()
                .unwrap();
            // Insert team component:
            self.connection
                .prepare(
                    "INSERT INTO team (id, team)
                VALUES (?, ?)",
                )
                .unwrap()
                .bind(1, i)
                .unwrap()
                .bind(2, team as i64)
                .unwrap()
                .next()
                .unwrap();
            // Insert universe component:
            self.connection
                .prepare(
                    "INSERT INTO universe (id, universe_id)
                VALUES (?, ?)",
                )
                .unwrap()
                .bind(1, i)
                .unwrap()
                .bind(2, universe_id as i64)
                .unwrap()
                .next()
                .unwrap();
            // Insert shooter component:
            self.connection
                .prepare(
                    "INSERT INTO shooter (id, shooter_cooldown)
                VALUES (?, ?)",
                )
                .unwrap()
                .bind(1, i)
                .unwrap()
                .bind(2, 0.0 as f64)
                .unwrap()
                .next()
                .unwrap();
        }
    }

    fn unload_universe(&mut self, universe_id: usize) {
        // Delete all entities in the universe:
        self.connection
            .prepare(
                "DELETE FROM entities WHERE id IN (SELECT id FROM universe WHERE universe_id = ?)",
            )
            .unwrap()
            .bind(1, universe_id as i64)
            .unwrap()
            .next()
            .unwrap();
        // Delete all position components:
        self.connection
            .prepare(
                "DELETE FROM position WHERE id IN (SELECT id FROM universe WHERE universe_id = ?)",
            )
            .unwrap()
            .bind(1, universe_id as i64)
            .unwrap()
            .next()
            .unwrap();
        // Delete all velocity components:
        self.connection
            .prepare(
                "DELETE FROM velocity WHERE id IN (SELECT id FROM universe WHERE universe_id = ?)",
            )
            .unwrap()
            .bind(1, universe_id as i64)
            .unwrap()
            .next()
            .unwrap();
        // Delete all team components:
        self.connection
            .prepare("DELETE FROM team WHERE id IN (SELECT id FROM universe WHERE universe_id = ?)")
            .unwrap()
            .bind(1, universe_id as i64)
            .unwrap()
            .next()
            .unwrap();
        // Delete all universe components:
        self.connection
            .prepare("DELETE FROM universe WHERE universe_id = ?")
            .unwrap()
            .bind(1, universe_id as i64)
            .unwrap()
            .next()
            .unwrap();
        // Delete all shooter components:
        self.connection
            .prepare(
                "DELETE FROM shooter WHERE id IN (SELECT id FROM universe WHERE universe_id = ?)",
            )
            .unwrap()
            .bind(1, universe_id as i64)
            .unwrap()
            .next()
            .unwrap();
    }

    fn on_click(&mut self, universe_id: usize, position: Vec2) {
        // TODO
    }
}
