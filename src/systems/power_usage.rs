use std::ops::{Deref, DerefMut};
use std::time::Instant;
use components::{DeltaTime, GatheringRate, PowerBar, ResourceCount, ResourceType, Resources,
                 StateChange, Text, Transform};
use state::play_state::PlayState;
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};
use systems::POWER_FACTOR;

const POWER_PER_TICK: i32 = 80;

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
        Read<'b, DeltaTime>,
        Read<'b, GatheringRate>,
        WriteStorage<'b, PowerBar>,
        Write<'b, Resources>,
        Write<'b, StateChange>,
        WriteStorage<'b, Text>,
        WriteStorage<'b, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            resource_count_storage,
            delta_time_storage,
            gathering_rate_storage,
            mut power_storage,
            mut resources_storage,
            mut state_change_storage,
            mut text_storage,
            mut transform_storage,
        ) = data;
        let resources: &mut Resources = resources_storage.deref_mut();

        self.frame_count += 1.0;
        let dt = delta_time_storage.deref().dt;
        let mut reset_frame_counter = false;

        let mut num_of_cites_to_power = 0;
        for (transform, power_bar) in (&mut transform_storage, &mut power_storage).join() {
            if self.frame_count * dt >= 5.0 {
                reset_frame_counter = true;
                self.instant = Instant::now();
                if power_bar.power_left > 0 {
                    power_bar.power_left -= POWER_PER_TICK;
                    if power_bar.power_left <= 0 {
                        let state_change: &mut StateChange = state_change_storage.deref_mut();
                        state_change.state = PlayState::get_name();
                        state_change.action = "restart".to_string();
                    }
                }

                let width = PowerBar::get_max_width()
                    * (power_bar.power_left as f32 / PowerBar::get_max_f32());
                transform.size.x = width as u16;
            }

            num_of_cites_to_power += 1;
        }

        if reset_frame_counter {
            self.frame_count = 0.0;
        }

        // 4 is the max
        if num_of_cites_to_power >= 4 {
            let gathering_rate = gathering_rate_storage.deref();
            let power_demands = (POWER_PER_TICK * 4) / POWER_FACTOR;

            if gathering_rate.coal / ResourceType::Coal.get_efficiency_rate()
                + gathering_rate.oil / ResourceType::Oil.get_efficiency_rate()
                + gathering_rate.solar / ResourceType::Solar.get_efficiency_rate()
                + gathering_rate.hydro / ResourceType::Hydro.get_efficiency_rate() >= power_demands
            {
                println!("~~~~~Meeting demands~~~~");
            }
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
