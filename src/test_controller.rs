use crate::brains::Brain;
use crate::challenges::Challenge;
use crate::ui::ui_settings::GuiSettings;
use plotters::prelude::*;
use std::collections::HashMap;

pub struct TestController {
    pub brain: Box<dyn Brain>,
    pub challenge: Box<dyn Challenge>,
    pub timings: HashMap<String, u128>,
    pub ticks: u32,
    pub universe_count: usize,
}

impl TestController {
    pub fn new(brain: Box<dyn Brain>, challenge: Box<dyn Challenge>) -> TestController {
        TestController {
            brain,
            challenge,
            timings: HashMap::new(),
            ticks: 0,
            universe_count: 1,
        }
    }
    pub fn init(&mut self, settings: &GuiSettings) {
        self.brain.init(&self.challenge.get_tick_systems());
        let time = crate::utils::time_it(|| {
            self.challenge
                .init(&mut *self.brain, self.universe_count, settings);
        });
        self.register_time(String::from("init"), time);
    }

    pub fn tick(&mut self, delta: f32, settings: &GuiSettings) {
        let systems = self.challenge.get_tick_systems();

        if settings.all_at_once {
            let time = crate::utils::time_it(|| {
                self.brain.tick_systems(delta, settings);
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
