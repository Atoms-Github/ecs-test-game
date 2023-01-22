#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

mod bench_utils;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use crate::bench_utils::benchmark;
use ecs_test_game::brains::brain_legion::BrainLegion;
use ecs_test_game::challenges::rts::ChallengeRts;
use ecs_test_game::simulation_settings::{BrainType, Challenge, SimSettings};
use ecs_test_game::test_controller::TestController;
use std::time::Duration;

criterion_group!(benches, color_closest);
criterion_main!(benches);

fn color_closest(c: &mut Criterion) {
    let mut group = c.benchmark_group("identical_entities");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(3));
    group.warm_up_time(Duration::from_millis(100));

    let tests = [BrainType::Legion, BrainType::SqlDuck, BrainType::SqlIte];
    let entity_counts = [100, 1000, 10000];
    let mut settings = SimSettings::default();
    settings.challenge_type = Challenge::IdenticalEntities;

    for entity_count in entity_counts {
        settings.entity_count = entity_count;
        for test in tests {
            settings.brain_type = test;
            benchmark(&mut group, &settings, 3);
        }
    }
}
