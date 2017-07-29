use std::time::Instant;
use components::{CurrentPower, PowerBar, Transform};
use specs::{ReadStorage, WriteStorage, Join, System};
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
        ReadStorage<'a, CurrentPower>,
        WriteStorage<'a, PowerBar>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (current_power_storage, mut power_storage, mut transform_storage) = data;

        let mut power_left = 0;
        for power_bar in (&mut power_storage).join() {
            power_left = power_bar.power_left;
            if math::get_mills(&self.instant.elapsed()) >= 0.25 {
                self.instant = Instant::now();
                power_bar.power_left -= 1;
                if power_bar.power_left == 0 {
                    // end scenario
                    panic!("lost!");
                }
            }
        }

        for (_, transform) in (&current_power_storage, &mut transform_storage).join() {
            let width = CurrentPower::get_max_with() as f32 * (power_left as f32 / 100.0);
            transform.size.x = width as u16;
        }
    }
}