use std::borrow::{Borrow, BorrowMut};
use crate::brains::com::*;
use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;
use crate::{MAP_SIZE, Point};
use ggez::graphics::Color;
use glam::*;
use legion::systems::CommandBuffer;
use legion::*;
use rand::Rng;
use crate::utils::FromTeam;

pub struct BrainLegionScheduled {
    world: World,
    schedule: Option<Schedule>,
}

// A function for making a new unit
pub fn make_unit(world: &mut World, pos: Vec2, vel: Vec2, team: usize, universe_id: usize) {
    let command_buffer = &mut CommandBuffer::new(world);
    let ent = command_buffer.push((
        PositionComp { pos },
        VelocityComp { vel },
        TeamComp { team },
        ColorComp {
            color: Color::from_team(team),
        },
        UniverseComp { universe_id },
        ShooterComp { cooldown: 0.0 },
    ));
    command_buffer.flush(world, &mut Resources::default());
}
fn make_projectile(buffer: &mut CommandBuffer, pos: Vec2, target: Vec2, universe_id: usize) {
    let vel = (target - pos).normalize() * 100.0;
    buffer.push((
        PositionComp { pos },
        VelocityComp { vel },
        ColorComp {
            color: Color::new(1.0, 1.0, 1.0, 1.0),
        },
        TimedLifeComp { time_left: 1.0 },
        UniverseComp { universe_id },
    ));
}

#[system(for_each)]
fn velocity(#[resource] dt: &f32, pos: &mut PositionComp, vel: &VelocityComp) {
    pos.pos += vel.vel * *dt;
}
#[system(for_each)]
fn acceleration(#[resource] dt: &f32, vel: &mut VelocityComp, acc: &AccelerationComp) {
    vel.vel += acc.acc * *dt;
}
#[system(for_each)]
fn map_edge(pos: &mut PositionComp) {
    pos.pos.x = pos.pos.x.rem_euclid(MAP_SIZE);
    pos.pos.y = pos.pos.y.rem_euclid(MAP_SIZE);
}
// Decrement the time left on all entities with a TimedLife component
#[system(for_each)]
fn update_timed_life(#[resource] dt: &f32, time: &mut TimedLifeComp) {
    time.time_left -= *dt;
}
// Shoot projectiles at the nearest enemy
#[system(for_each)]
fn shoot(
    #[resource] dt: &f32,
    #[resource] other_entities: &Vec<(PositionComp, TeamComp, UniverseComp)>,
    #[resource] settings: &GuiSettings,
    pos: &PositionComp,
    team: &TeamComp,
    spawner: &mut ShooterComp,
    universe: &UniverseComp,
    buffer: &mut CommandBuffer,
) {
    spawner.cooldown -= *dt;
    if spawner.cooldown > 0.0 {
        return;
    }
    spawner.cooldown = rand::thread_rng().gen_range(0.25..0.75);
    let mut closest_dist = f32::MAX;
    let mut closest_pos = Vec2::ZERO;
    for (other_pos, other_team, other_universe) in other_entities.iter() {
        if other_team.team == team.team || other_universe.universe_id != universe.universe_id {
            continue;
        }
        let dist = (pos.pos - other_pos.pos).length();
        if dist < closest_dist {
            closest_dist = dist;
            closest_pos = other_pos.pos;
        }
    }
    if closest_dist < settings.meet_distance {
        make_projectile(buffer, pos.pos, closest_pos, universe.universe_id);
    }
}
#[system(for_each)]
fn paint_nearest(
    #[resource] other_entities: &Vec<(PositionComp, ColorComp)>,
    #[resource] settings: &GuiSettings,
    pos: &PositionComp,
    color: &mut ColorComp,
) {
    let mut closest_dist = f32::MAX;
    let mut closest_color = &ColorComp {
        color: Color::new(0.0, 0.0, 0.0, 1.0),
    };
    for (other_pos, other_color) in other_entities.iter() {
        let dist = (pos.pos - other_pos.pos).length();
        if dist < closest_dist {
            closest_dist = dist;
            closest_color = &other_color;
        }
    }
    color.blend(closest_color, &settings);
}
// Delete entities that have expired
#[system(for_each)]
fn delete_expired(time: &TimedLifeComp, entity: &Entity, command_buffer: &mut CommandBuffer) {
    if time.time_left <= 0.0 {
        command_buffer.remove(*entity);
    }
}
impl BrainLegionScheduled {
    pub fn new() -> Self {
        let mut world = World::default();
        Self {
            world,
            schedule: None,
        }
    }
}
impl Brain for BrainLegionScheduled {
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    ) {
        make_unit(&mut self.world, position, velocity, team, universe_id);
    }


    fn add_entity_vel_dot(&mut self, position: Point, velocity: Point) {
        self.world.push((
            PositionComp { pos: position },
            VelocityComp { vel: velocity },
            ColorComp {
                color: Color::new(1.0, 1.0, 1.0, 1.0),
            },
        ));
    }

    fn add_entity_positional_dummy(&mut self, position: Point, color: Color) {
        self.world.push((
            PositionComp { pos: position },
            ColorComp {
                color,
            },
        ));

    }

    fn get_entities(&mut self, universe_id: usize) -> Vec<(Point, Color)> {
        let mut query = <(Read<PositionComp>, Read<ColorComp>, Read<UniverseComp>)>::query();
        let mut entities = Vec::new();
        for (pos, color, universe) in query.iter(&self.world) {
            if universe.universe_id == universe_id {
                entities.push((pos.pos, color.color));
            }
        }
        entities

    }

    fn init_systems(&mut self, systems: &Vec<SystemType>) {
        let mut schedule = Schedule::builder();
        for system in systems.iter() {
            match system {
                SystemType::Velocity => schedule.add_system(velocity_system()),
                SystemType::Acceleration => schedule.add_system(acceleration_system()),
                SystemType::MapEdge => schedule.add_system(map_edge_system()),
                SystemType::UpdateTimedLife => schedule.add_system(update_timed_life_system()),
                SystemType::Shoot => schedule.add_system(shoot_system()),
                SystemType::DeleteExpired => schedule.add_system(delete_expired_system()),
                SystemType::PaintNearest => schedule.add_system(paint_nearest_system()),

            };
        }
        self.schedule = Some(schedule.build());
    }

    fn get_tick_all_at_once(&self) -> bool {
        true
    }

    fn tick_systems(&mut self, delta: f32, settings: &GuiSettings) {
        let mut resources = Resources::default();
        resources.insert(delta);
        resources.insert(settings.clone());

        let mut other_entities = Vec::new();
        let mut query = <(&PositionComp, &TeamComp, &UniverseComp)>::query();
        for (pos, team, universe) in query.iter(&self.world) {
            other_entities.push((*pos, *team, *universe));
        }
        resources.insert(other_entities);
        resources.insert(settings.clone());

        let schedule = self.schedule.as_mut().unwrap();
        schedule.execute(&mut self.world, &mut resources);
    }

    fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &GuiSettings) {
        panic!("Should run multi")
    }

    fn get_name(&self) -> String {
        String::from("Legion scheduled")
    }
}
