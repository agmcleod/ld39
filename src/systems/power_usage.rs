use std::ops::{Deref, DerefMut};
use std::time::Instant;
use components::{CurrentPower, DeltaTime, PowerBar, ResourceCount, Resources, StateChange, Text, Transform};
use state::play_state::PlayState;
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct PowerUsage {
    instant: Instant,
    frame_count: f32,
}

impl PowerUsage {
    pub fn new() -> PowerUsage {
        PowerUsage {
            instant: Instant::now(),
            frame_count: 0.0,
        }
    }
}

impl<'b> System<'b> for PowerUsage {
    type SystemData = (
        ReadStorage<'b, ResourceCount>,
        ReadStorage<'b, CurrentPower>,
        Read<'b, DeltaTime>,
        WriteStorage<'b, PowerBar>,
        Write<'b, Resources>,
        Write<'b, StateChange>,
        WriteStorage<'b, Text>,
        WriteStorage<'b, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            resource_count_storage,
            current_power_storage,
            delta_time_storage,
            mut power_storage,
            mut resources_storage,
            mut state_change_storage,
            mut text_storage,
            mut transform_storage,
        ) = data;
        let resources: &mut Resources = resources_storage.deref_mut();

        let mut power_left = 0;
        self.frame_count += 1.0;
        let dt = delta_time_storage.deref().dt;
        for power_bar in (&mut power_storage).join() {
            power_left = power_bar.power_left;
            if self.frame_count * dt >= 1.0 {
                self.frame_count = 0.0;
                self.instant = Instant::now();
                if power_bar.power_left > 0 {
                    power_bar.power_left -= 100;
                    if power_bar.power_left <= 0 {
                        let state_change: &mut StateChange = state_change_storage.deref_mut();
                        state_change.state = PlayState::get_name();
                        state_change.action = "restart".to_string();
                    }
                }
            }
        }

        for (_, transform) in (&current_power_storage, &mut transform_storage).join() {
            let width =
                CurrentPower::get_max_with() as f32 * (power_left as f32 / PowerBar::get_max());
            transform.size.x = width as u16;
        }

        for (resource_count, text) in (&resource_count_storage, &mut text_storage).join() {
            let new_text = format!(
                "{}",
                resources.get_amount_for_type(&resource_count.resource_type)
            );
            text.set_text(new_text);
        }
    }
}
