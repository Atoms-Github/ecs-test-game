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

criterion_group!(benches, identical_entities);
criterion_main!(benches);

fn identical_entities(c: &mut Criterion) {
	let mut group = c.benchmark_group("identical_entities");
	group.sample_size(10);
	group.measurement_time(Duration::from_secs(3));
	group.warm_up_time(Duration::from_millis(100));

	let mut settings = SimSettings::default();
	settings.challenge_type = Challenge::ImageEditing;

	for entity_count in (1..10).map(|x| x * 1000) {
		settings.entity_count = entity_count;
		for test in [
			BrainType::LegionDupey,
			BrainType::LegionCounted,
			BrainType::SqlDuck,
			BrainType::SqlIte,
		] {
			settings.brain_type = test;
			benchmark(&mut group, &settings, 3);
		}
	}
}
