use crate::ui::ui_settings::GuiSettings;
use crate::Point;
use ggez::graphics::Color;
use crate::simulation_settings::SimSettings;

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionComp {
    pub pos: Point,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorComp {
    pub blue: f32,
}
impl ColorComp {
    pub fn blend(&mut self, other: &ColorComp, settings: &SimSettings) {
        if settings.paint_speed != 0.0 {
            self.blue = (self.blue + other.blue / (settings.paint_speed + 1.0)) % 1.0;
        }
    }
}
pub struct ExportEntity {
    pub position: Point,
    pub blue: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TeamComp {
    pub team: usize,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccelerationComp {
    pub acc: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityComp {
    pub vel: Point,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShooterComp {
    pub cooldown: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimedLifeComp {
    pub time_left: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UniverseComp {
    pub universe_id: usize,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageComp {
    sound: ggez::
}