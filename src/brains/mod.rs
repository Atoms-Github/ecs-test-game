use std::borrow::Cow;
use std::fmt;
use std::time::Duration;

use ggez::graphics::Color;

use crate::brains::com::{BlobComp, ExportEntity, TeamComp};
use crate::simulation_settings::SimSettings;
use crate::ui::ui_settings::GuiSettings;
use crate::Point;

pub mod brain_legion;
pub mod com;
pub mod ecs_concepts;
// pub mod sparsey;
pub mod brain_rc_ecs;
pub mod sql_brains;
pub mod sql_interfaces;

pub trait Brain {
	fn add_entity_unit(&mut self, position: Point, velocity: Point, team: usize, universe_id: usize);
	fn add_entity(&mut self, position: Point, velocity: Option<Point>, blue: f32);
	fn add_entity_blob(&mut self, position: Point, blob: &BlobComp, blue: f32, team: Option<usize>);

	fn get_entities(&mut self, universe_id: usize) -> Vec<ExportEntity>;
	fn get_image(&mut self, entity_id: u64) -> Cow<Vec<u8>>;

	fn init(&mut self, systems: &Vec<SystemType>);

	fn tick_systems(&mut self, delta: f32, settings: &SimSettings, systems: &Vec<SystemType>);
	fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &SimSettings);

	fn get_name(&self) -> String;
	fn clone_box(&self) -> Box<dyn Brain>;
}
#[derive(Debug, PartialEq, Clone)]
pub enum SystemType {
	Velocity,
	Acceleration,
	MapEdge,
	UpdateTimedLife,
	Shoot,
	DeleteExpired,
	PaintNearest,
	EditTeamOneImage,
	EditTeamOneColor,
}
impl SystemType {
	pub fn get_name(&self) -> String {
		return format!("{:?}", self);
	}
}
