#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

mod bench_utils;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ecs_test_game::brains::brain_legion::BrainLegion;
use ecs_test_game::challenges::ch_units_shooting::ChallengeRts;
use ecs_test_game::simulation_settings::{BrainType, Challenge, SimSettings};
use ecs_test_game::test_controller::TestController;

use crate::bench_utils::benchmark;

criterion_group!(benches, units_shooting);
criterion_main!(benches);

fn units_shooting(c: &mut Criterion) {
	let mut group = c.benchmark_group("units_shooting");
	group.sample_size(10);
	group.measurement_time(Duration::from_secs(3));
	group.warm_up_time(Duration::from_millis(100));

	let tests = [
		BrainType::LegionDupey,
		BrainType::SqlDuck,
		BrainType::SqlIte,
	];
	let entity_counts = [5, 10, 20, 50, 70, 85, 100, 130, 160, 200, 230, 250];
	let mut settings = SimSettings::default();
	settings.challenge_type = Challenge::UnitsShooting;

	for entity_count in entity_counts {
		settings.entity_count = entity_count;
		for test in tests {
			settings.brain_type = test;
			benchmark(&mut group, &settings, 3);
		}
	}
}
