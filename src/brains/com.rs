use ggez::graphics::Color;
use crate::Point;
use crate::ui::ui_settings::GuiSettings;

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionComp {
    pub pos: Point,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorComp {
    pub color: Color,
}
impl ColorComp{
    pub fn blend(&mut self, other: &ColorComp, settings: &GuiSettings){

        if settings.blend_speed != 0.0{

            self.color.r = (self.color.r + other.color.r / (settings.blend_speed + 1.0)) % 1.0;
        }
    }
}
pub struct ExportEntity{
    pub pos: Point,
    pub color: Color,
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