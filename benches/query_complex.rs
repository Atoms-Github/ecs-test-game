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
use ecs_test_game::challenges::rts::ChallengeRts;
use ecs_test_game::simulation_settings::{BrainType, Challenge, SimSettings};
use ecs_test_game::test_controller::TestController;

use crate::bench_utils::benchmark;

criterion_group!(benches, query_complex);
criterion_main!(benches);

fn query_complex(c: &mut Criterion) {
	let mut group = c.benchmark_group("query_complex");
	group.sample_size(10);
	group.measurement_time(Duration::from_secs(3));
	group.warm_up_time(Duration::from_millis(100));

	let mut settings = SimSettings::default();
	settings.challenge_type = Challenge::QueryChallenge;

	for entity_count in (1..2).map(|x| x * 2) {
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
