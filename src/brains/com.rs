use std::hash::Hash;

use ggez::graphics::Color;

use crate::simulation_settings::SimSettings;
use crate::ui::ui_settings::GuiSettings;
use crate::Point;

// a component is any type that is 'static, sized, send and sync
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionComp {
	pub pos: Point,
}

impl Hash for PositionComp {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.pos.x.to_bits().hash(state);
		self.pos.y.to_bits().hash(state);
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorComp {
	pub blue: f32,
}

impl Hash for ColorComp {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.blue.to_bits().hash(state);
	}
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
	pub blue:     f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub struct TeamComp {
	pub team: usize,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccelerationComp {
	pub acc: Point,
}

impl Hash for AccelerationComp {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.acc.x.to_bits().hash(state);
		self.acc.y.to_bits().hash(state);
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityComp {
	pub vel: Point,
}

impl Hash for VelocityComp {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.vel.x.to_bits().hash(state);
		self.vel.y.to_bits().hash(state);
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShooterComp {
	pub cooldown: f32,
}

impl Hash for ShooterComp {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.cooldown.to_bits().hash(state);
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimedLifeComp {
	pub time_left: f32,
}

impl Hash for TimedLifeComp {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.time_left.to_bits().hash(state);
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub struct UniverseComp {
	pub universe_id: usize,
}
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct BlobComp {
	pub blob: Vec<u8>,
}
