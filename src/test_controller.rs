use crate::brains::Brain;
use crate::challenges::Challenge;
use std::collections::HashMap;

pub struct TestController
{
    pub brain: Box<dyn Brain>,
    pub challenge: Box<dyn Challenge>,
    pub timings: HashMap<&'static str, u128>,
    pub ticks: u32,
    pub universe_count: usize,
}



impl TestController
{
    pub fn new(brain: Box<dyn Brain>, challenge: Box<dyn Challenge>) -> TestController
    {
        TestController {
            brain,
            challenge,
            timings: HashMap::new(),
            ticks: 0,
            universe_count: 1
        }
}
    fn init(&mut self) {
        let time = crate::utils::time_it(|| {
            self.challenge.init(&mut *self.brain, self.universe_count);
        });
        self.brain.init_systems(&self.challenge.get_tick_systems());
        self.register_time("init", time);
    }

    fn tick(&mut self) {
        let systems = self.challenge.get_tick_systems();
        if self.brain.get_tick_all_at_once() {
            let time = crate::utils::time_it(|| {
                self.brain.tick_systems();
            });
            self.register_time("ALL_SYSTEMS", time);
        } else {
            for system in systems {
                let time = crate::utils::time_it(|| {
                    self.brain.tick_system(&system);
                });
                self.register_time(system.as_string(), time);
            }
        }
        self.ticks += 1;
    }
    fn register_time(&mut self, key: &'static str, time: u128) {
        let entry = self.timings.entry(key).or_insert(0);
        *entry += time;

    }

}
