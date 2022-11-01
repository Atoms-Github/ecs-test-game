use crate::rts::{Position, TimedLife, Velocity, STARTING_VELOCITY, WORLD_SIZE};
use crate::GameImplementation;
use crate::GuiSettings;
use glam::*;
use legion::systems::CommandBuffer;
use legion::*;

pub struct GameLegion {
    world: World,
}

impl GameLegion {
    pub fn new() -> GameLegion {
        let mut world = World::default();

        GameLegion { world }
    }
    pub fn generate_world(&mut self, universe_id: usize) {
        for _ in 0..100 {
            self.world.push((
                Position {
                    pos: Vec2::new(
                        rand::random::<f32>() * WORLD_SIZE,
                        rand::random::<f32>() * WORLD_SIZE,
                    ),
                },
                Velocity {
                    vel: Vec2::new(
                        (0.5 - rand::random::<f32>()) * STARTING_VELOCITY,
                        (0.5 - rand::random::<f32>()) * STARTING_VELOCITY,
                    ),
                },
                // UniverseComp { id: universe_id },
            ));
        }
    }
}
impl GameImplementation for GameLegion {
    fn update(&mut self, ctx: &mut ggez::Context, settings: &GuiSettings) {
        let mut query = <(Read<Velocity>, Write<Position>)>::query();
        query.for_each_mut(&mut self.world, |(vel, mut pos)| {
            pos.pos += vel.vel * ggez::timer::delta(ctx).as_secs_f32();
            if pos.pos.x < 0.0 {
                pos.pos.x = WORLD_SIZE;
            }
            if pos.pos.x > WORLD_SIZE {
                pos.pos.x = 0.0;
            }
            if pos.pos.y < 0.0 {
                pos.pos.y = WORLD_SIZE;
            }
            if pos.pos.y > WORLD_SIZE {
                pos.pos.y = 0.0;
            }
        });
        let mut positions = vec![];
        let mut query = <(Read<Position>, Read<Velocity>)>::query();

        query.for_each(&self.world, |(pos, vel)| {
            positions.push(pos.pos);
        });
        let mut buffer = CommandBuffer::new(&mut self.world);
        for unit_a in <(Write<Velocity>, Read<Position>)>::query().iter_mut(&mut self.world) {
            for unit_b in positions.iter() {
                let distance = unit_a.1.pos.distance(*unit_b);
                if distance > 0. && distance < settings.meet_distance {
                    let entity = buffer.push(());

                    buffer.add_component(entity, Position { pos: unit_a.1.pos });
                    buffer.add_component(entity, TimedLife { time_left: 0.0 });
                }
                if unit_a.0.vel.length() > settings.max_speed {
                    unit_a.0.vel = Vec2::ZERO;
                }
                unit_a.0.vel *= settings.drag_coefficient;
            }
        }
        buffer.flush(&mut self.world, &mut Resources::default());
    }

    fn get_unit_positions(&self, universe_id: usize) -> Vec<Vec2> {
        let mut query = <(Read<Position>)>::query();
        let mut positions = Vec::new();
        for position in query.iter(&self.world) {
            positions.push(position.pos);
        }
        positions
    }
}
