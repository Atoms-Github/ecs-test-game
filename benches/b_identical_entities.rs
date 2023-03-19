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
use ecs_test_game::brains::brain_legion::{BrainLegion, BrainLegionCounted};
use ecs_test_game::challenges::ch_units_shooting::ChallengeRts;
use ecs_test_game::simulation_settings::{BrainType, Challenge, SimSettings};
use ecs_test_game::test_controller::TestController;

use crate::bench_utils::benchmark;

criterion_group!(benches, b_identical_entities);
criterion_main!(benches);

fn b_identical_entities(c: &mut Criterion) {
	let mut group = c.benchmark_group("b_identical_entities");
	group.sample_size(10);
	group.measurement_time(Duration::from_secs(3));
	group.warm_up_time(Duration::from_millis(100));

	let mut settings = SimSettings::default();
	settings.challenge_type = Challenge::IdenticalEntities;

	let entity_counts = [5000, 20_000];
	// let entity_counts = [500, 5000, 15_000, 35_000, 50_000, 100_000];

	for entity_count in entity_counts {
		settings.entity_count = entity_count;
		for test in [
			// BrainType::Legion,
			// BrainType::LegionCounted,
			BrainType::Duck_DB,
			// BrainType::Sqlite_DB,
			BrainType::Rc_Ecs,
		] {
			settings.brain_type = test;
			benchmark(&mut group, &settings, 10);
		}
	}
}
