use ggez::graphics::Color;
use glam::*;
use legion::systems::CommandBuffer;
use legion::*;
use rand::Rng;
use crate::brains::com::*;
use crate::brains::{Brain, SystemType};
use crate::brains::com::{ColorComp, Position, Shooter, Team, TimedLife, UniverseComp, Velocity};
use crate::{MAP_SIZE, Point};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::team_to_color;

pub struct BrainLegionSequential {
    world: World,
}
impl Brain for BrainLegionSequential{
    fn add_entity_unit(&mut self, position: Point, velocity: Point, team: usize, universe_id: usize) {
        self.world.push((
            Position{pos: position},
            Velocity{vel: velocity},
            Team{team},
            UniverseComp{ universe_id},
            ColorComp{color: team_to_color(team)},
            Shooter{ cooldown: 0.0},
        ));
    }

    fn add_entity_vel_dot(&mut self, position: Point, velocity: Point) {
        self.world.push((
            Position{pos: position},
            Velocity{vel: velocity},
            ColorComp{color: Color::new(1.0, 1.0, 1.0, 1.0)},
        ));
    }

    fn add_entity_positional_dummy(&mut self, position: Point) {
        self.world.push((
            Position{pos: position},
            ColorComp{color: Color::new(1.0, 1.0, 1.0, 1.0)},
        ));
    }

    fn get_entities(&mut self) -> Vec<(Point, Color)> {
        let mut entities = Vec::new();
        let mut query = <(&Position, &ColorComp)>::query();
        for (pos, color) in query.iter(&self.world) {
            entities.push((pos.pos, color.color));
        }
        entities
    }

    fn init_systems(&mut self, systems: &Vec<SystemType>) {
        // None
    }

    fn get_tick_all_at_once(&self) -> bool {
        false
    }

    fn tick_systems(&mut self, delta: f32) {
        // None
    }

    fn tick_system(&mut self, system: &SystemType, delta: f32) {
        match  system{
            SystemType::VELOCITY => {
                let mut query = <(&mut Position, &Velocity)>::query();
                for (mut pos, vel) in query.iter_mut(&mut self.world) {
                    pos.pos += vel.vel * delta;
                }
            }
            SystemType::ACCELERATION => {
                let mut query = <(&mut Velocity, &Acceleration)>::query();
                for (mut vel, acc) in query.iter_mut(&mut self.world) {
                    vel.vel += acc.acc * delta;
                }
            }
            SystemType::MAP_EDGE => {
                let mut query = <(&mut Position, &mut Velocity)>::query();
                for (mut pos, mut vel) in query.iter_mut(&mut self.world) {
                    if pos.pos.x < 0.0 {
                        pos.pos.x = 0.0;
                        vel.vel.x = -vel.vel.x;
                    }
                    if pos.pos.x > MAP_SIZE {
                        pos.pos.x = MAP_SIZE;
                        vel.vel.x = -vel.vel.x;
                    }
                    if pos.pos.y < 0.0 {
                        pos.pos.y = 0.0;
                        vel.vel.y = -vel.vel.y;
                    }
                    if pos.pos.y > MAP_SIZE {
                        pos.pos.y = MAP_SIZE;
                        vel.vel.y = -vel.vel.y;
                    }
                }
            }
            SystemType::UPDATE_TIMED_LIFE => {
                let mut query = <(&mut TimedLife)>::query();
                for (mut timed_life) in query.iter_mut(&mut self.world) {
                    timed_life.time_left -= delta;
                }
            }
            SystemType::SHOOT => {
                let command_buffer = &mut CommandBuffer::new(&mut self.world);
                let mut query = <(&Shooter, &Position, &Team)>::query();
                for (shooter, pos, team) in query.iter(&self.world) {
                    if shooter.cooldown <= 0.0 {
                        // Shoot towards near enemy
                        let mut query = <(&Position, &Team)>::query();
                        let mut near_enemy = None;
                        let mut near_enemy_dist = 100000.0;
                        for (enemy_pos, enemy_team) in query.iter(&self.world) {
                            if enemy_team.team != team.team {
                                let dist = (enemy_pos.pos - pos.pos).length();
                                if dist < near_enemy_dist {
                                    near_enemy = Some(enemy_pos.pos);
                                    near_enemy_dist = dist;
                                }
                            }
                        }
                        if let Some(enemy_pos) = near_enemy {
                            let vel = (enemy_pos - pos.pos).normalize() * 5.0;
                            command_buffer.push((
                                Position{pos: pos.pos},
                                Velocity{vel},
                                Team{team: team.team},
                                ColorComp{color: Color::new(1.0, 1.0, 1.0, 1.0)},
                                TimedLife{time_left: 1.0},
                            ));
                        }

                    }
                }
                command_buffer.flush(&mut self.world, &mut Resources::default());
                // Process cooldown
                let mut query = <(&mut Shooter)>::query();
                for (mut shooter) in query.iter_mut(&mut self.world) {
                    if shooter.cooldown <= 0.0 {
                        shooter.cooldown = 3.0;
                    }
                    shooter.cooldown -= delta;
                }
            }
            SystemType::DELETE_EXPIRED => {
                let mut command_buffer = CommandBuffer::new(&self.world);
                let mut query = <(Entity,&TimedLife)>::query();
                for (entity, timed_life) in query.iter(&self.world) {
                    if timed_life.time_left <= 0.0 {
                        command_buffer.remove(*entity);
                    }
                }
                command_buffer.flush(&mut self.world, &mut Resources::default());
            }
        }
    }

    fn get_name(&self) -> String {
        String::from("Legion Sequential")
    }
}
impl BrainLegionSequential {
    pub fn new() -> Self {
        Self{
            world: Default::default(),
        }
    }
}

// A function for making a new unit
fn make_unit(world: &mut World, pos: Vec2, vel: Vec2, team: usize, universe_id: usize) {
    let command_buffer = &mut CommandBuffer::new(world);
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
        UniverseComp { universe_id },
    ));
}
#[system(for_each)]
fn shoot(
    #[resource] dt: &f32,
    #[resource] other_entities: &Vec<(Position, Team, UniverseComp)>,
    #[resource] settings: &GuiSettings,
    pos: &Position,
    team: &Team,
    spawner: &mut Shooter,
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


