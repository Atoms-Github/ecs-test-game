use ggez::input::mouse::position;
use rand::Rng;

use crate::brains::{Brain, SystemType};
use crate::challenges::ChallengeTrait;
use crate::simulation_settings::SimSettings;
use crate::ui::ui_settings::GuiSettings;
use crate::{Point, MAP_SIZE};

#[derive(Clone)]
pub struct ChallengeRts {}
impl ChallengeTrait for ChallengeRts {
	fn init(&mut self, brain: &mut dyn Brain, universe_count: usize, settings: &SimSettings) {
		let mut rand = rand::thread_rng();

		const SPEED: f32 = 30.0;
		// Create units. Distribute them evenly across universes.
		let mut universe_id = 0;
		for _ in 0..settings.entity_count {
			let position = Point::new(rand.gen_range(0.0..MAP_SIZE), rand.gen_range(0.0..MAP_SIZE));
			let velocity = Point::new(rand.gen_range(-SPEED..SPEED), rand.gen_range(-SPEED..SPEED));
			let team = rand.gen_range(0..3);
			brain.add_entity_unit(position, velocity, team, universe_id);
			universe_id = (universe_id + 1) % universe_count;
		}
	}

	fn get_tick_systems(&self) -> Vec<SystemType> {
		return vec![
			SystemType::Acceleration,
			SystemType::Velocity,
			SystemType::MapEdge,
			SystemType::UpdateTimedLife,
			SystemType::Shoot,
			SystemType::DeleteExpired,
		];
	}

	fn clone_box(&self) -> Box<dyn ChallengeTrait> {
		Box::new(self.clone())
	}
}
