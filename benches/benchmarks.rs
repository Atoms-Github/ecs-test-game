#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ecs_test_game::basic_legion::BasicLegion;
use ecs_test_game::gamesqlite::*;
use ecs_test_game::performance_map_legion::PerfMapLegion;
use ecs_test_game::relation_per_component::RelationPerComponent;
use ecs_test_game::rts::*;
use std::time::Duration;

criterion_group!(benches, rts_benchmark);
criterion_main!(benches);

fn rts_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("game_update");
    const UNIVERSES: usize = 10;
    const ENTITY_COUNT: i64 = 100;
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(3));
    group.warm_up_time(Duration::from_millis(100));
    group.bench_function("BasicLegion", |b| {
        // Init game state
        let mut world = BasicLegion::new();
        for i in 0..UNIVERSES {
            world.load_universe(i, ENTITY_COUNT);
        }
        b.iter(move || {
            world.update(1. / 60., &GuiSettings::default());
        });
    });
    group.bench_function("PerfMapLegion", |b| {
        // Init game state
        let mut world = PerfMapLegion::new();
        for i in 0..UNIVERSES {
            world.load_universe(i, ENTITY_COUNT);
        }
        b.iter(move || {
            world.update(1. / 60., &GuiSettings::default());
        });
    });

    group.bench_function("VerySlowSQL", |b| {
        // Init game state
        let mut world = RelationPerComponent::new();
        for i in 0..UNIVERSES {
            world.load_universe(i, ENTITY_COUNT);
        }
        b.iter(move || {
            world.update(1. / 60., &GuiSettings::default());
        });
    });
    group.bench_function("Sqlite", |b| {
        // Init game state
        let mut world = SqlIte::new();
        for i in 0..UNIVERSES {
            world.load_universe(i, ENTITY_COUNT);
        }
        b.iter(move || {
            world.update(1. / 60., &GuiSettings::default());
        });
    });
}
