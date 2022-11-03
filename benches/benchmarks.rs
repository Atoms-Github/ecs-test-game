use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ecs_test_game::game_legion::GameLegion;
use ecs_test_game::main::GuiSettings;
use std::time::Duration;

criterion_group!(benches, rts_benchmark);
criterion_main!(benches);

fn rts_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("game_update");
    const UNIVERSES: usize = 10;
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(10));
    group.warm_up_time(Duration::from_millis(20));
    group.bench_function("legion", |b| {
        // Init game state
        let mut world = GameLegion::new();
        for i in 0..UNIVERSES {
            world.load_universe(i);
        }
        b.iter(move || {
            world.update(1. / 60., &GuiSettings::default());
        });
    });
}
