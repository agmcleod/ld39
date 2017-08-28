use std::ops::DerefMut;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use components::{ResourceCount, CurrentPower, PowerBar, Resources, Text, Transform, WinCount};
use state::StateManager;
use specs::{FetchMut, ReadStorage, WriteStorage, Join, System};
use utils::math;

pub struct PowerUsage {
    instant: Instant,
    state_manager: Arc<Mutex<StateManager>>,
}

impl PowerUsage {
    pub fn new(state_manager: Arc<Mutex<StateManager>>) -> PowerUsage {
        PowerUsage{
            instant: Instant::now(),
            state_manager: state_manager,
        }
    }
}

impl<'b> System<'b> for PowerUsage {
    type SystemData = (
        ReadStorage<'b, ResourceCount>,
        ReadStorage<'b, CurrentPower>,
        FetchMut<'b, Resources>,
        WriteStorage<'b, PowerBar>,
        WriteStorage<'b, Text>,
        WriteStorage<'b, Transform>,
        ReadStorage<'b, WinCount>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (resource_count_storage, current_power_storage, mut resources_storage, mut power_storage, mut text_storage, mut transform_storage, win_count_storage) = data;
        let resources: &mut Resources = resources_storage.deref_mut();

        let mut done = false;
        for win_count in (&win_count_storage).join() {
            if win_count.count == 0 {
                done = true;
            }
        }

        if !done {
            let mut power_left = 0;
            for power_bar in (&mut power_storage).join() {
                power_left = power_bar.power_left;
                if math::get_mills(&self.instant.elapsed()) >= 0.25 {
                    self.instant = Instant::now();
                    power_bar.power_left -= 1;
                    if power_bar.power_left == 0 {
                        self.state_manager.lock().unwrap().restart_next_frame = true;
                    }
                }
            }

            for (_, transform) in (&current_power_storage, &mut transform_storage).join() {
                let width = CurrentPower::get_max_with() as f32 * (power_left as f32 / 100.0);
                transform.size.x = width as u16;
            }
        }

        for (resource_count, text) in (&resource_count_storage, &mut text_storage).join() {
            let new_text = format!("{}", resources.get_amount_for_type(&resource_count.resource_type));
            if new_text != text.text {
                text.set_text(new_text);
            }
        }
    }
}