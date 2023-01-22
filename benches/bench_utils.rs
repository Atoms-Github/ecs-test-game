use std::fmt::format;
use criterion::measurement::WallTime;
use criterion::{BenchmarkGroup, BenchmarkId, Criterion};
use rand::Rng;
use ecs_test_game::simulation_settings::{BrainType, SimSettings};
use ecs_test_game::test_controller::TestController;

pub fn benchmark(group: &mut BenchmarkGroup<WallTime>, settings: &SimSettings, frame_count: usize) {
    let mut controller = TestController::gen_test_controller(settings);
    let mut entity_count = settings.entity_count;

    let name = format!("{}, {}", settings.brain_type.to_string(), rand::thread_rng().gen_range(0..5000));
    group.bench_with_input(
        BenchmarkId::new(settings.brain_type.to_string(), entity_count),
        &mut entity_count,
        |b, _| {
            b.iter(|| {
                for _ in 0..frame_count {
                    controller.tick(0.016, settings);
                }
            })
        },
    );
}
