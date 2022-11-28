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
pub mod db_interface;
pub mod test_controller;
pub mod thread_controller;
pub mod ui;
pub mod utils;

pub type Point = glam::Vec2;
pub const MAP_SIZE: f32 = 800.0;
