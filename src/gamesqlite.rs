use crate::rts::*;

use crate::rts::STARTING_VELOCITY;
use crate::rts::WORLD_SIZE;
use ggez::graphics::Color;
use glam::*;
use legion::systems::CommandBuffer;
use legion::*;
use rand::Rng;
use sqlite::*;

pub struct SqlIte {
    connection: Connection,
}

impl SqlIte {
    pub fn new() -> SqlIte {
        let connection = open(":memory:").unwrap();

        connection
            .execute(
                "CREATE TABLE entities (
            id INTEGER PRIMARY KEY,
            position_x REAL,
            position_y REAL,
            velocity_x REAL,
            velocity_y REAL,
            team INTEGER,
            universe_id INTEGER,
            shooter_cooldown REAL,
            timed_life REAL
        )",
            )
            .unwrap();
        SqlIte { connection }
    }
}
impl GameImplementation for SqlIte {
    fn update(&mut self, dt: f32, settings: &GuiSettings) {
        let mut resources = Resources::default();
        // Start transaction:
        self.connection.execute("BEGIN").unwrap();
        // Update x
        self.connection
            .prepare("UPDATE entities SET position_x = position_x + velocity_x * ?;")
            .unwrap()
            .bind(1, dt as f64)
            .unwrap()
            .next()
            .unwrap();
        // Update y
        self.connection
            .prepare("UPDATE entities SET position_y = position_y + velocity_y * ?;")
            .unwrap()
            .bind(1, dt as f64)
            .unwrap()
            .next()
            .unwrap();

        // Update timed life:
        self.connection
            .prepare("UPDATE entities SET timed_life = timed_life - ?")
            .unwrap()
            .bind(1, dt as f64)
            .unwrap()
            .next()
            .unwrap();
        // Update shooter cooldown:
        self.connection
            .prepare("UPDATE entities SET shooter_cooldown = shooter_cooldown - ?")
            .unwrap()
            .bind(1, dt as f64)
            .unwrap()
            .next()
            .unwrap();
        // Delete expired:
        self.connection
            .execute("DELETE FROM entities WHERE timed_life < 0")
            .unwrap();
        // Shoot:
        self.connection
            .execute(
                "INSERT INTO entities (position_x, position_y, velocity_x, velocity_y, team, universe_id, timed_life)
            SELECT position_x, position_y, velocity_x, velocity_y, team, universe_id, 1.0
            FROM entities
            WHERE shooter_cooldown < 0",
            )
            .unwrap();
        // Update shooter cooldown:
        self.connection
            .execute("UPDATE entities SET shooter_cooldown = 0.5 WHERE shooter_cooldown < 0")
            .unwrap();



        self.connection.execute("COMMIT").unwrap();
    }

    fn get_unit_positions(&self, universe_id: usize) -> Vec<(Vec2, Color)> {
        let mut positions_and_teams = Vec::new();
        let mut statement = self
            .connection
            .prepare("SELECT position_x, position_y, team FROM entities WHERE universe_id = ?")
            .unwrap()
            .bind(1, universe_id as i64)
            .unwrap();

        while let Ok(State::Row) = statement.next() {
            let x = statement.read::<f64>(0).unwrap();
            let y = statement.read::<f64>(1).unwrap();
            let team = statement.read::<i64>(2).unwrap();
            let color = match team {
                0 => Color::from_rgb(255, 0, 0),
                1 => Color::from_rgb(0, 255, 0),
                2 => Color::from_rgb(0, 0, 255),
                _ => Color::from_rgb(255, 255, 255),
            };

            positions_and_teams.push((Vec2::new(x as f32, y as f32), color));
        }
        positions_and_teams
    }

    fn load_universe(&mut self, universe_id: usize, entity_count: i64) {
        for i in 0..entity_count {
            let mut rng = rand::thread_rng();
            let pos = Vec2::new(
                rng.gen_range(0.0..WORLD_SIZE),
                rng.gen_range(0.0..WORLD_SIZE),
            );
            let vel = Vec2::new(
                rng.gen_range(0.0..STARTING_VELOCITY),
                rng.gen_range(0.0..STARTING_VELOCITY),
            );
            let team = rng.gen_range(0..=2);
            // Insert an entity into the database using format!():
            self.connection
                .execute(&format!(
                    "INSERT INTO entities (
                position_x,
                position_y,
                velocity_x,
                velocity_y,
                team,
                universe_id,
                shooter_cooldown
            ) VALUES (
                {}, {}, {}, {}, {}, {}, {}
            )",
                    pos.x, pos.y, vel.x, vel.y, team, universe_id, 1.0
                ))
                .unwrap();
        }
    }

    fn unload_universe(&mut self, universe_id: usize) {
        self.connection
            .execute(&format!(
                "DELETE FROM entities WHERE universe_id = {}",
                universe_id
            ))
            .unwrap();
    }

    fn on_click(&mut self, universe_id: usize, position: Vec2) {
        // TODO
    }
}
