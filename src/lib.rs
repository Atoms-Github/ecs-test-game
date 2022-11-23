#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

mod brains;
mod challenges;
pub mod db_interface;
mod test_controller;
mod thread_controller;
pub mod utils;

pub type Point = glam::Vec2;
