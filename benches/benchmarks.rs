#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::time::Duration;
use ecs_test_game::brains::legion_sequential::BrainLegionSequential;
use ecs_test_game::challenges::rts::ChallengeRts;
use ecs_test_game::test_controller::TestController;
use ecs_test_game::ui::ui_settings::BrainType::LegionScheduled;
use ecs_test_game::ui::ui_settings::ChallengeType::Rts;
use ecs_test_game::ui::ui_settings::GuiSettings;

criterion_group!(benches, rts_benchmark);
criterion_main!(benches);

fn rts_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("rts");
    const ENTITY_COUNT: usize = 100;
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(3));
    group.warm_up_time(Duration::from_millis(100));
    group.bench_function("Legion Scheduled", |b| {
        let brain = Box::new(BrainLegionSequential::new());
        let challenge = Box::new(ChallengeRts {
            units_count: ENTITY_COUNT,
        });
        let mut test_controller = TestController::new(brain, challenge);
        b.iter(move || {
            test_controller.tick(0.16, &GuiSettings::new());
        });
    });
    group.bench_function("Legion Sequential", |b| {
        let brain = Box::new(BrainLegionSequential::new());
        let challenge = Box::new(ChallengeRts {
            units_count: ENTITY_COUNT,
        });
        let mut test_controller = TestController::new(brain, challenge);
        b.iter(move || {
            test_controller.tick(0.16, &GuiSettings::new());
        });
    });
}
