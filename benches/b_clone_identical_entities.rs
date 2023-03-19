#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

mod bench_utils;
use std::time::Duration;

use criterion::measurement::WallTime;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion};
use ecs_test_game::brains::brain_legion::{BrainLegion, BrainLegionCounted};
use ecs_test_game::challenges::ch_units_shooting::ChallengeRts;
use ecs_test_game::simulation_settings::{BrainType, Challenge, SimSettings};
use ecs_test_game::test_controller::TestController;
use rand::Rng;

use crate::bench_utils::{benchmark, gen_test_ctx};

criterion_group!(benches, b_clone_identical_entities);
criterion_main!(benches);

fn b_clone_identical_entities(c: &mut Criterion) {
	let mut group = c.benchmark_group("b_clone_identical_entities");
	group.sample_size(10);
	group.measurement_time(Duration::from_secs(3));
	group.warm_up_time(Duration::from_millis(100));

	let mut settings = SimSettings::default();
	settings.challenge_type = Challenge::IdenticalEntities;

	let entity_counts = [1000, 3000];
	// let entity_counts = [10, 100, 500, 2500, 5_000];

	for entity_count in entity_counts {
		settings.entity_count = entity_count;
		for test in [
			BrainType::Legion,
			// BrainType::LegionCounted,
			// BrainType::Duck_DB,
			// BrainType::Sqlite_DB,
			BrainType::Rc_Ecs,
		] {
			settings.brain_type = test;
			benchmark_clone(&mut group, &settings, 10);
		}
	}
}
pub fn benchmark_clone(group: &mut BenchmarkGroup<WallTime>, settings: &SimSettings, frame_count: usize) {
	let mut ctx = gen_test_ctx();
	let mut controller = TestController::gen_test_controller(&mut ctx, settings);
	let mut entity_count = settings.entity_count;

	let name = format!(
		"Cloning {}, {}",
		settings.brain_type.to_string(),
		rand::thread_rng().gen_range(0..5000)
	);
	group.bench_with_input(
		BenchmarkId::new(settings.brain_type.to_string(), entity_count),
		&mut entity_count,
		|b, _| {
			b.iter(|| {
				let new = controller.clone();
				black_box(new);
			})
		},
	);
}
