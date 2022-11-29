use egui::pos2;
use ggez::graphics::Color;
use glam::*;
use legion::systems::CommandBuffer;
use legion::*;
use rand::Rng;
use crate::brains::com::*;
use crate::brains::{Brain, SystemType};
use crate::brains::com::{ColorComp, PositionComp, ShooterComp, TeamComp, TimedLifeComp, UniverseComp, VelocityComp};
use crate::{MAP_SIZE, Point};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::{FromTeam};

pub struct BrainLegionSequential {
    world: World,
}
impl Brain for BrainLegionSequential{
    fn add_entity_unit(&mut self, position: Point, velocity: Point, team: usize, universe_id: usize) {
        self.world.push((
            PositionComp {pos: position},
            VelocityComp {vel: velocity},
            TeamComp {team},
            UniverseComp{ universe_id},
            ColorComp{color: Color::from_team(team)},
            ShooterComp { cooldown: 0.0},
        ));
    }

    fn add_entity_vel_dot(&mut self, position: Point, velocity: Point) {
        self.world.push((
            PositionComp {pos: position},
            VelocityComp {vel: velocity},
            ColorComp{color: Color::new(1.0, 1.0, 1.0, 1.0)},
        ));
    }

    fn add_entity_positional_dummy(&mut self, position: Point, color: Color) {
        self.world.push((
            PositionComp {pos: position},
            ColorComp{color},
        ));
    }

    fn get_entities(&mut self, universe_id: usize) -> Vec<(Point, Color)> {
        let mut entities = Vec::new();
        let mut query = <(&PositionComp, &ColorComp)>::query();
        for (pos, color) in query.iter(&self.world) {
            entities.push((pos.pos, color.color));
        }
        entities
    }

    fn init_systems(&mut self, systems: &Vec<SystemType>) {
    }

    fn get_tick_all_at_once(&self) -> bool {
        false
    }

    fn tick_systems(&mut self, delta: f32, settings: &GuiSettings) {
        panic!("Should run singles")
    }

    fn tick_system(&mut self, system: &SystemType, delta: f32) {
        match  system{
            SystemType::PaintNearest => {
                let mut buffer = CommandBuffer::new(&self.world);
                let mut query = <(&Entity, &PositionComp, &ColorComp)>::query();
                let mut query_inner = <(&PositionComp, &ColorComp)>::query();
                let mut nearest = None;
                let mut nearest_distance = 10000000.0;
                for (entity, pos, color) in query.iter(&self.world) {
                    let mut color = color.clone();
                    for (pos2, color2) in query_inner.iter(&self.world) {
                        // Find nearest
                        let dist = (pos.pos - pos2.pos).length();
                        if dist < nearest_distance {
                            nearest_distance = dist;
                            nearest = Some(color2);
                        }
                    }
                    color.blend(nearest.unwrap());
                    buffer.add_component(*entity, color);
                }
                buffer.flush(&mut self.world, &mut Resources::default());
            }
            SystemType::Velocity => {
                let mut query = <(&mut PositionComp, &VelocityComp)>::query();
                for (mut pos, vel) in query.iter_mut(&mut self.world) {
                    pos.pos += vel.vel * delta;
                }
            }
            SystemType::Acceleration => {
                let mut query = <(&mut VelocityComp, &AccelerationComp)>::query();
                for (mut vel, acc) in query.iter_mut(&mut self.world) {
                    vel.vel += acc.acc * delta;
                }
            }
            SystemType::MapEdge => {
                let mut query = <(&mut PositionComp)>::query();
                for (mut pos) in query.iter_mut(&mut self.world) {
                    pos.pos.x = pos.pos.x.rem_euclid(MAP_SIZE);
                    pos.pos.y = pos.pos.y.rem_euclid(MAP_SIZE);
                }
            }
            SystemType::UpdateTimedLife => {
                let mut query = <(&mut TimedLifeComp)>::query();
                for (mut timed_life) in query.iter_mut(&mut self.world) {
                    timed_life.time_left -= delta;
                }
            }
            SystemType::Shoot => {
                let command_buffer = &mut CommandBuffer::new(&mut self.world);
                let mut query = <(&ShooterComp, &PositionComp, &TeamComp)>::query();
                for (shooter, pos, team) in query.iter(&self.world) {
                    if shooter.cooldown <= 0.0 {
                        // Shoot towards near enemy
                        let mut query = <(&PositionComp, &TeamComp)>::query();
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
                            let vel = (enemy_pos - pos.pos).normalize() * 50.0;
                            command_buffer.push((
                                PositionComp {pos: pos.pos},
                                VelocityComp {vel},
                                TeamComp {team: team.team},
                                ColorComp{color: Color::new(1.0, 1.0, 1.0, 1.0)},
                                TimedLifeComp {time_left: 1.0},
                            ));
                        }

                    }
                }
                command_buffer.flush(&mut self.world, &mut Resources::default());
                // Process cooldown
                let mut query = <(&mut ShooterComp)>::query();
                for (mut shooter) in query.iter_mut(&mut self.world) {
                    if shooter.cooldown <= 0.0 {
                        shooter.cooldown = 3.0;
                    }
                    shooter.cooldown -= delta;
                }
            }
            SystemType::DeleteExpired => {
                let mut command_buffer = CommandBuffer::new(&self.world);
                let mut query = <(Entity,&TimedLifeComp)>::query();
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