use std::time::Instant;
use components::Power;
use specs::{WriteStorage, Join, System};
use utils::math;

pub struct PowerUsage {
    instant: Instant,
}

impl PowerUsage {
    pub fn new() -> PowerUsage {
        PowerUsage{
            instant: Instant::now(),
        }
    }
}

impl<'a> System<'a> for PowerUsage {
    type SystemData = (
        WriteStorage<'a, Power>
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut power_storage = data;

        for power in (&mut power_storage).join() {
            if math::get_mills(&self.instant.elapsed()) >= 0.25 {
                self.instant = Instant::now();
                power.power_left -= 1;
                if power.power_left == 0 {
                    // end scenario
                    panic!("lost!");
                }
            }
        }
    }
}