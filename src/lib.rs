#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

extern crate core;

pub mod brains;
pub mod challenges;
pub mod rc_ecs;
pub mod simulation_settings;
pub mod test_controller;
pub mod thread_controller;
pub mod ui;
pub mod utils;

pub type Point = glam::Vec2;
pub const MAP_SIZE: f32 = 600.0;
pub const PROJECTILE_LIFETIME: f32 = 2.0;
pub const SHOOT_SPEED: f32 = 0.030;
