#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use ecs_test_game::brains::brain_legion::BrainLegion;
use ecs_test_game::challenges::rts::ChallengeRts;
use ecs_test_game::simulation_settings::SimSettings;
use ecs_test_game::test_controller::TestController;
use ecs_test_game::ui::ui_settings::{BrainType, Challenge, GuiSettings};
use std::time::Duration;

criterion_group!(benches, nearest_line);
criterion_main!(benches);

fn spatial_array(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_array");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(3));
    group.warm_up_time(Duration::from_millis(100));

    let tests = [BrainType::LegionDupey, BrainType::SqlDuck];
    let entity_counts = [5, 20, 50, 100, 500, 1000];
    let mut settings = SimSettings::default();

    settings.challenge_type = Challenge::SpacialArray {};
    for entity_count in entity_counts {
        settings.entity_count = entity_count;
        for test in tests {
            settings.brain_type = test;
            benchmark(&mut group, &settings, 3);
        }
    }
}
