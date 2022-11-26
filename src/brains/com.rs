use ggez::graphics::Color;
use crate::Point;

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub pos: Point,
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
pub struct Acceleration {
    pub acc: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub vel: Point,
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
    pub universe_id: usize,
}
