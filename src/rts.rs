use ggez::graphics::Color;
use glam::{f32, Vec2};

pub const WORLD_SIZE: f32 = 700.0;
pub const STARTING_VELOCITY: f32 = 30.0;

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub pos: Vec2,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorComp {
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Team {
    pub team: usize,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub vel: Vec2,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shooter {
    pub cooldown: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimedLife {
    pub time_left: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniverseComp {
    pub id: usize,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Comp1 {
    pub value: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Comp2 {
    pub value: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Comp3 {
    pub value: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Comp4 {
    pub value: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Comp5 {
    pub value: f32,
}

pub trait GameImplementation {
    fn update(&mut self, dt: f32, settings: &GuiSettings);
    fn get_unit_positions(&self, unierse_id: usize) -> Vec<(glam::Vec2, Color)>;
    fn load_universe(&mut self, universe_id: usize);
    fn unload_universe(&mut self, unierse_id: usize);
    fn on_click(&mut self, universe_id: usize, position: Vec2);
}

#[derive(Clone, Copy, Default)]
pub struct GuiSettings {
    pub meet_distance: f32,
    pub universe: usize,
    pub requested_universe_count: usize,
}
