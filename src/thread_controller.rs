use crate::brains::Brain;
use crate::challenges::Challenge;
use crate::test_controller::TestController;

pub struct ThreadController {
    test_controller: TestController,
}
impl
    ThreadController
{
    pub fn process(
        self,
        universe_count: usize,
        universes_per_thread: usize,
        tick_count: u128,
    ) -> u128 {
      //  let mut thread_count = universe_count / universes_per_thread;
      //  let mut threads = vec![];
      //  let mut brain = self.test_controller.brain.clone_box();
      //  let challenge = self.test_controller.challenge.clone_box();
      //  brain.init_systems(&challenge.get_tick_systems());
      //  for _ in 0..thread_count {
      //      let brain = brain.clone_box();
      //      let challenge = challenge.clone_box();
      //      threads.push(std::thread::spawn(move || {
      //  //        let mut test_controller = TestController::new(brain.clone(), challenge.clone());
      //  //        test_controller.init();
      //  //        for _ in 0..tick_count {
      //  //            test_controller.tick();
      //  //        }
      //      }));
      //  }
      //  crate::utils::time_it(|| {
      //      for thread in threads {
      //          thread.join().unwrap();
      //      }
      //  })
        0
    }
}
