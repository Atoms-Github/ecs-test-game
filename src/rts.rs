use glam::{f32, Vec2};

pub const WORLD_SIZE: f32 = 700.0;
pub const STARTING_VELOCITY: f32 = 50.0;

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub pos: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub vel: Vec2,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub vel: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimedLife {
    pub time_left: f32,
}

struct UniverseComp {
    pub id: usize,
}
