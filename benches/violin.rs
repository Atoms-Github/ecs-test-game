#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ecs_test_game::brains::brain_legion::BrainLegion;
use ecs_test_game::challenges::rts::ChallengeRts;
use ecs_test_game::test_controller::TestController;
use ecs_test_game::ui::ui_settings::{BrainType, ChallengeType, GuiSettings};
use std::time::Duration;

criterion_group!(benches, nearest_violin);
criterion_main!(benches);

fn nearest_violin(c: &mut Criterion) {
    let mut group = c.benchmark_group("nearest");
    let mut settings = GuiSettings {
        shoot_distance: 30.0,
        view_universe: 0,
        universe_count: 0,
        entity_count: 0,
        blend_speed: 0.0,
        brain_type: BrainType::Legion,
        challenge_type: ChallengeType::GetNearest,
        all_at_once: true,
    };
    const FRAMES: usize = 100;
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(3));
    group.warm_up_time(Duration::from_millis(100));
    group.bench_function("Legion", |b| {
        let mut controller = TestController::gen_test_controller(&settings);
        b.iter(move || {
            for i in 0..FRAMES {
                controller.tick(0.016, &mut settings);
            }
        });
    });
    settings.brain_type = BrainType::SqlIte;
    group.bench_function("Sqlite", |b| {
        let mut controller = TestController::gen_test_controller(&settings);
        b.iter(move || {
            for i in 0..FRAMES {
                controller.tick(0.016, &mut settings);
            }
        });
    });
    settings.brain_type = BrainType::SqlDuck;
    group.bench_function("SqlDuck", |b| {
        let mut controller = TestController::gen_test_controller(&settings);
        b.iter(move || {
            for i in 0..FRAMES {
                controller.tick(0.016, &mut settings);
            }
        });
    });
}
