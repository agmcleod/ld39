use std::ops::DerefMut;
use std::time::Instant;
use components::{CoalCount, CurrentPower, PowerBar, Resources, Text, Transform};
use specs::{FetchMut, ReadStorage, WriteStorage, Join, System};
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

impl<'b> System<'b> for PowerUsage {
    type SystemData = (
        ReadStorage<'b, CoalCount>,
        ReadStorage<'b, CurrentPower>,
        FetchMut<'b, Resources>,
        WriteStorage<'b, PowerBar>,
        WriteStorage<'b, Text>,
        WriteStorage<'b, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (coal_count_storage, current_power_storage, mut resources_storage, mut power_storage, mut text_storage, mut transform_storage) = data;
        let resources: &mut Resources = resources_storage.deref_mut();

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

        for (_, text) in (&coal_count_storage, &mut text_storage).join() {
            let new_text = format!("{}", resources.coal);
            if new_text != text.text {
                text.set_text(new_text);
            }
        }
    }
}