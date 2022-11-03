use crate::rts::{
    ColorComp, Comp1, Comp2, Comp3, Position, Spawner, Team, TimedLife, UniverseComp, Velocity,
    STARTING_VELOCITY, WORLD_SIZE,
};
use crate::GameImplementation;
use crate::GuiSettings;
use ggez::graphics::Color;
use glam::*;
use legion::systems::CommandBuffer;
use legion::*;
use rand::Rng;

pub struct GameLegion {
    world: World,
    schedule: Schedule,
}

// A function for making a new unit
pub fn make_unit(world: &mut World, pos: Vec2, vel: Vec2, team: usize, universe_id: usize) {
    let command_buffer = &mut CommandBuffer::new(world);
    let ent = command_buffer.push((
        Position { pos },
        Velocity { vel },
        Team { team },
        ColorComp {
            color: match team {
                0 => Color::new(1.0, 0.0, 0.0, 1.0),
                1 => Color::new(0.0, 1.0, 0.0, 1.0),
                2 => Color::new(0.0, 0.0, 1.0, 1.0),
                _ => Color::new(0.0, 0.0, 0.0, 1.0),
            },
        },
        UniverseComp { id: universe_id },
        Spawner { cooldown: 0.0 },
    ));

    if rand::thread_rng().gen_bool(0.5) {
        command_buffer.add_component(ent, Comp1 { value: 0.0 });
    }
    if rand::thread_rng().gen_bool(0.5) {
        command_buffer.add_component(ent, Comp2 { value: 1.0 })
    }
    command_buffer.flush(world, &mut Resources::default());
}
fn make_projectile(buffer: &mut CommandBuffer, pos: Vec2, target: Vec2, universe_id: usize) {
    let vel = (target - pos).normalize() * 100.0;
    buffer.push((
        Position { pos },
        Velocity { vel },
        ColorComp {
            color: Color::new(1.0, 1.0, 1.0, 1.0),
        },
        TimedLife { time_left: 1.0 },
        UniverseComp { id: universe_id },
    ));
}

#[system(for_each)]
fn update_positions(#[resource] dt: &f32, pos: &mut Position, vel: &Velocity) {
    pos.pos += vel.vel * *dt;
}
#[system(for_each)]
fn map_edge(pos: &mut Position) {
    pos.pos.x = pos.pos.x.rem_euclid(WORLD_SIZE);
    pos.pos.y = pos.pos.y.rem_euclid(WORLD_SIZE);
}
// Decrement the time left on all entities with a TimedLife component
#[system(for_each)]
fn update_timed_life(#[resource] dt: &f32, time: &mut TimedLife) {
    time.time_left -= *dt;
}
// Shoot projectiles at the nearest enemy
#[system(for_each)]
fn shoot(
    #[resource] dt: &f32,
    #[resource] other_entities: &Vec<(Position, Team, UniverseComp)>,
    #[resource] settings: &GuiSettings,
    pos: &Position,
    team: &Team,
    spawner: &mut Spawner,
    universe: &UniverseComp,
    buffer: &mut CommandBuffer,
) {
    spawner.cooldown -= *dt;
    if spawner.cooldown > 0.0 {
        return;
    }
    spawner.cooldown = 0.5;
    let mut closest_dist = f32::MAX;
    let mut closest_pos = Vec2::ZERO;
    for (other_pos, other_team, other_universe) in other_entities.iter() {
        if other_team.team == team.team || other_universe.id != universe.id {
            continue;
        }
        let dist = (pos.pos - other_pos.pos).length();
        if dist < closest_dist {
            closest_dist = dist;
            closest_pos = other_pos.pos;
        }
    }
    if closest_dist < settings.meet_distance {
        make_projectile(buffer, pos.pos, closest_pos, universe.id);
    }
}

// Delete entities that have expired
#[system(for_each)]
fn delete_expired(time: &TimedLife, entity: &Entity, command_buffer: &mut CommandBuffer) {
    if time.time_left <= 0.0 {
        command_buffer.remove(*entity);
    }
}
#[system(for_each)]
fn waste_of_time1(pos: &Position, comp1: &mut Comp1) {
    comp1.value += 1.;
    comp1.value += pos.pos.x;
}

#[system(for_each)]
fn waste_of_time2(pos: &Position, comp3: &mut Comp3, comp2: &Comp2) {
    comp3.value += comp2.value;
}
impl GameLegion {
    pub fn new() -> GameLegion {
        let mut world = World::default();
        let mut schedule = Schedule::builder();
        schedule.add_system(update_positions_system());
        schedule.add_system(map_edge_system());
        schedule.add_system(update_timed_life_system());
        schedule.add_system(shoot_system());
        schedule.add_system(delete_expired_system());

        for _ in 0..5 {
            schedule.add_system(waste_of_time1_system());
        }
        for _ in 0..5 {
            schedule.add_system(waste_of_time2_system());
        }
        GameLegion {
            world,
            schedule: schedule.build(),
        }
    }
}
impl GameImplementation for GameLegion {
    fn update(&mut self, ctx: &mut ggez::Context, settings: &GuiSettings) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let mut resources = Resources::default();
        resources.insert(dt);
        resources.insert(settings.clone());

        let mut other_entities = Vec::new();
        let mut query = <(&Position, &Team, &UniverseComp)>::query();
        for (pos, team, universe) in query.iter(&self.world) {
            other_entities.push((*pos, *team, *universe));
        }
        resources.insert(other_entities);

        self.schedule.execute(&mut self.world, &mut resources);
    }

    fn get_unit_positions(&self, universe_id: usize) -> Vec<(Vec2, Color)> {
        let mut query = <(Read<Position>, Read<ColorComp>, Read<UniverseComp>)>::query();
        let mut positions = Vec::new();
        for position in query.iter(&self.world) {
            if position.2.id == universe_id {
                positions.push((position.0.pos, position.1.color));
            }
        }
        positions
    }

    fn load_universe(&mut self, universe_id: usize) {
        for i in 0..100 {
            let pos = Vec2::new(
                rand::random::<f32>() * WORLD_SIZE,
                rand::random::<f32>() * WORLD_SIZE,
            );
            let vel = Vec2::new(
                rand::random::<f32>() * STARTING_VELOCITY,
                rand::random::<f32>() * STARTING_VELOCITY,
            );
            let team = rand::random::<usize>() % 3;
            make_unit(&mut self.world, pos, vel, team, universe_id);
        }
    }

    fn unload_universe(&mut self, unierse_id: usize) {
        let mut query = <(Read<UniverseComp>, Entity)>::query();
        let command_buffer = &mut CommandBuffer::new(&self.world);
        for (universe, entity) in query.iter(&self.world) {
            if universe.id == unierse_id {
                command_buffer.remove(*entity);
            }
        }
        command_buffer.flush(&mut self.world, &mut Resources::default());
    }

    fn on_click(&mut self, universe_id: usize, position: Vec2) {
        // Move all entities in the blue team in this universe towards the clicked point
        let mut query = <(
            Read<Position>,
            Read<Team>,
            Read<UniverseComp>,
            Write<Velocity>,
        )>::query();
        for (pos, team, universe, mut vel) in query.iter_mut(&mut self.world) {
            if team.team == 1 && universe.id == universe_id {
                vel.vel = (position - pos.pos).normalize() * STARTING_VELOCITY;
            }
        }
    }
}
