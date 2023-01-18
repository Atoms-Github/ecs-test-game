use criterion::measurement::WallTime;
use criterion::{BenchmarkGroup, BenchmarkId, Criterion};
use ecs_test_game::simulation_settings::{BrainType, SimSettings};
use ecs_test_game::test_controller::TestController;

pub fn benchmark(group: &mut BenchmarkGroup<WallTime>, settings: &SimSettings, frame_count: usize) {
    let mut controller = TestController::gen_test_controller(settings);
    group.bench_with_input(
        BenchmarkId::new(settings.brain_type, entity_count),
        entity_count,
        |b, _| {
            b.iter(|| {
                for _ in 0..frame_count {
                    controller.tick(0.016, settings);
                }
            })
        },
    );
}
