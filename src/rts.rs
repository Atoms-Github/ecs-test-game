use ggez::graphics::Color;
use glam::{f32, Vec2};

pub const WORLD_SIZE: f32 = 700.0;
pub const STARTING_VELOCITY: f32 = 20.0;

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
pub struct Spawner {
    pub cooldown: f32
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


