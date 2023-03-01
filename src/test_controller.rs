use crate::brains::brain_legion::{BrainLegion, BrainLegionCounted, BrainLegionDupey};
use crate::brains::sql_brains::brain_sql::BrainSql;
use crate::brains::sql_brains::sql_flat_table::BrainSqlFlatTable;
use crate::brains::sql_interfaces::duckdb::InterfaceDuckDB;
use crate::brains::sql_interfaces::sqlite::InterfaceSqlite;
use crate::brains::sql_interfaces::SqlInterface;
use crate::brains::Brain;
use crate::challenges::get_nearest::ChallengeGetNearest;
use crate::challenges::rts::ChallengeRts;
use crate::challenges::spacial_array::ChallengeSpatialArray;
use crate::challenges::ChallengeTrait;
use crate::simulation_settings::{BrainType, Challenge, SimSettings};
use crate::ui::ui_settings::GuiSettings;
use plotters::prelude::*;
use std::collections::HashMap;
use crate::brains::sql_interfaces::postgresql::InterfacePostgres;
use crate::challenges::identical_entities::ChallengeIdenticalEntities;

pub struct TestController {
    pub brain: Box<dyn Brain>,
    pub challenge: Box<dyn ChallengeTrait>,
    pub timings: HashMap<String, u128>,
    pub ticks: u32,
    pub universe_count: usize,
}

impl TestController {
    pub fn gen_test_controller(settings: &SimSettings) -> TestController {
        let new_brain: Box<dyn Brain> = match settings.brain_type {
            BrainType::LegionDupey => Box::new(BrainLegion::<BrainLegionDupey>::new()),
            BrainType::LegionCounted => Box::new(BrainLegion::<BrainLegionCounted>::new()),
            BrainType::SqlDuck => Box::new(BrainSql::new(
                BrainSqlFlatTable::new(),
                InterfaceDuckDB::new(),
            )),
            BrainType::SqlIte => Box::new(BrainSql::new(
                BrainSqlFlatTable::new(),
                InterfaceSqlite::new(),
            )),
            BrainType::SqlPostgres => Box::new(BrainSql::new(
                BrainSqlFlatTable::new(),
                InterfacePostgres::new(),
            )),

        };
        let new_challenge: Box<dyn ChallengeTrait> = match settings.challenge_type {
            Challenge::SpacialArray => Box::new(ChallengeSpatialArray {
                has_velocity_fraction: 1.0,
                dupe_entity_fraction: 1.0,
                unique_velocity_fraction: 0.001,
            }),
            Challenge::Rts  => Box::new(ChallengeRts {}),
            Challenge::PaintClosest  => Box::new(ChallengeGetNearest {}),
            Challenge::IdenticalEntities => Box::new(ChallengeIdenticalEntities {}),
        };

        let mut controller = TestController::new(new_brain, new_challenge);
        controller.init(settings);
        controller
    }
    pub fn new(brain: Box<dyn Brain>, challenge: Box<dyn ChallengeTrait>) -> TestController {
        TestController {
            brain,
            challenge,
            timings: HashMap::new(),
            ticks: 0,
            universe_count: 1,
        }
    }
    pub fn init(&mut self, settings: &SimSettings) {
        self.brain.init(&self.challenge.get_tick_systems());
        let time = crate::utils::time_it(|| {
            self.challenge
                .init(&mut *self.brain, self.universe_count, settings);
        });
        self.register_time(String::from("init"), time);
    }

    pub fn tick(&mut self, delta: f32, settings: &SimSettings) {
        let systems = self.challenge.get_tick_systems();

        if settings.all_at_once {
            let time = crate::utils::time_it(|| {
                self.brain.tick_systems(delta, settings, &systems);
            });
            self.register_time(String::from("ALL_SYSTEMS"), time);
        } else {
            for system in systems {
                let time = crate::utils::time_it(|| {
                    self.brain.tick_system(&system, delta, settings);
                });
                self.register_time(system.get_name(), time);
            }
        }
        self.ticks += 1;
    }
    fn register_time(&mut self, key: String, time: u128) {
        let entry = self.timings.entry(key).or_insert(0);
        *entry += time;
    }
    pub fn save_graph(&self, path: &str) {
        let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption("System timings", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..self.ticks, 0..100000000)
            .unwrap();
        chart.configure_mesh().draw().unwrap();
    }
}
