use ggez::graphics::Color;
use glam::{f32, Vec2};

pub const WORLD_SIZE: f32 = 700.0;
pub const STARTING_VELOCITY: f32 = 30.0;


pub trait GameImplementation {
    fn update(&mut self, dt: f32, settings: &GuiSettings);
    fn get_unit_positions(&self, unierse_id: usize) -> Vec<(glam::Vec2, Color)>;
    fn load_universe(&mut self, universe_id: usize, entity_count: i64);
    fn unload_universe(&mut self, unierse_id: usize);
    fn on_click(&mut self, universe_id: usize, position: Vec2);
}

