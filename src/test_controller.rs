use crate::brains::Brain;
use crate::challenges::Challenge;
use std::collections::HashMap;

pub struct TestController<B, C>
{
    pub brain: B,
    pub challenge: C,
    pub timings: HashMap<&'static str, u128>,
    pub ticks: u32,
}
impl<B: Brain + Clone, C: Challenge + Clone> Clone for TestController<B, C> {
    fn clone(&self) -> Self {
        Self {
            brain: self.brain.clone(),
            challenge: self.challenge.clone(),
            timings: self.timings.clone(),
            ticks: self.ticks,
        }
    }
}

impl<B, C> TestController<B, C>
where
    B: Brain,
    C: Challenge,
{
    pub fn new(brain: B, challenge: C) -> Self {
        Self {
            brain,
            challenge,
            timings: HashMap::new(),
            ticks: 0,
        }
    }
    fn register_time(&mut self, key: &'static str, time: u128) {
        let entry = self.timings.entry(key).or_insert(0);
        *entry += time;
    }

    pub fn init(&mut self) {
        let time = crate::utils::time_it(|| {
            self.challenge.init(&mut self.brain);
        });
        self.register_time("init", time);
    }

    pub fn tick(&mut self) {
        let systems = self.challenge.get_tick_systems();
        if B::get_tick_all_at_once() {
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
}
