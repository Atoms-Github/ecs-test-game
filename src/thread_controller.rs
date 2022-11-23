use crate::brains::Brain;
use crate::challenges::Challenge;
use crate::test_controller::TestController;

pub struct ThreadController<B, C> {
    test_controller: TestController<B, C>,
}
impl<B: 'static + Brain + Send + Clone, C: 'static + Challenge + Send + Clone>
    ThreadController<B, C>
{
    pub fn process(
        self,
        universe_count: usize,
        universes_per_thread: usize,
        tick_count: u128,
    ) -> u128 {
        let mut thread_count = universe_count / universes_per_thread;
        let mut threads = vec![];
        let mut brain = self.test_controller.brain.clone();
        let challenge = self.test_controller.challenge.clone();
        brain.init_systems(&C::get_tick_systems());
        for _ in 0..thread_count {
            let brain = brain.clone();
            let challenge = challenge.clone();
            threads.push(std::thread::spawn(move || {
                let mut test_controller = TestController::new(brain.clone(), challenge.clone());
                test_controller.init();
                for _ in 0..tick_count {
                    test_controller.tick();
                }
            }));
        }
        crate::utils::time_it(|| {
            for thread in threads {
                thread.join().unwrap();
            }
        })
    }
}
