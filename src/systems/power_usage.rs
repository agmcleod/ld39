use components::{CityPowerState, Color, DeltaTime, EntityLookup, GatheringRate, PowerBar,
                 ResourceCount, Resources, StateChange, Text, Transform};
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};
use state::play_state::PlayState;
use std::ops::{Deref, DerefMut};
use std::time::Instant;
use systems::{logic, POWER_FACTOR, TICK_RATE};

pub struct PowerUsage {
    instant: Instant,
    power_consumption_timer: f32,
}

impl PowerUsage {
    pub fn new() -> PowerUsage {
        PowerUsage {
            instant: Instant::now(),
            power_consumption_timer: 0.0,
        }
    }
}

impl<'b> System<'b> for PowerUsage {
    type SystemData = (
        Write<'b, CityPowerState>,
        WriteStorage<'b, Color>,
        Read<'b, DeltaTime>,
        Read<'b, EntityLookup>,
        Read<'b, GatheringRate>,
        WriteStorage<'b, PowerBar>,
        ReadStorage<'b, ResourceCount>,
        Write<'b, Resources>,
        Write<'b, StateChange>,
        WriteStorage<'b, Text>,
        WriteStorage<'b, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut city_power_state_storage,
            mut color_storage,
            delta_time_storage,
            entity_lookup_storage,
            gathering_rate_storage,
            mut power_bar_storage,
            resource_count_storage,
            mut resources_storage,
            mut state_change_storage,
            mut text_storage,
            mut transform_storage,
        ) = data;
        let resources: &mut Resources = resources_storage.deref_mut();

        let dt = delta_time_storage.deref().dt;
        self.power_consumption_timer += dt;
        let mut reset_frame_counter = false;

        let city_power_state = city_power_state_storage.deref_mut();
        for (transform, power_bar) in (&mut transform_storage, &mut power_bar_storage).join() {
            if self.power_consumption_timer >= TICK_RATE {
                reset_frame_counter = true;
                self.instant = Instant::now();
                if power_bar.power_left > 0 {
                    power_bar.power_left -= power_bar.power_per_tick;
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
        }

        if reset_frame_counter {
            self.power_consumption_timer = 0.0;

            let lookup = entity_lookup_storage.deref();

            let power_gain_entity = lookup.entities.get(&"power_gain_text".to_string()).unwrap();
            let gathering_rate = gathering_rate_storage.deref();

            // technically singular, so we could maybe make this a resource
            // or at least lookup via entity
            let power_demands = (&power_bar_storage)
                .join()
                .fold(0, |sum, power_bar| sum + power_bar.power_per_tick)
                / POWER_FACTOR;

            let total_gathering_rate = logic::get_total_gathering_rate(&gathering_rate);

            if total_gathering_rate - power_demands > 0 {
                city_power_state.current_city_count += 1;
                for power_bar in (&mut power_bar_storage).join() {
                    let mut per_tick = power_bar.power_per_tick;
                    // each city is more demanding
                    for n in 0..city_power_state.current_city_count {
                        per_tick += 15 * ((n as i32) + 1);
                    }
                    power_bar.power_per_tick = per_tick;
                }
            }

            let powering_text = if city_power_state.current_city_count > 1 {
                format!(
                    "Power: {}\n{} cities",
                    total_gathering_rate - power_demands,
                    city_power_state.current_city_count
                )
            } else {
                format!("Power: {}", total_gathering_rate - power_demands)
            };

            text_storage.get_mut(*power_gain_entity).unwrap().text = powering_text;

            color_storage
                .insert(
                    *power_gain_entity,
                    Color(if total_gathering_rate >= power_demands {
                        [0.0, 0.6, 0.0, 1.0]
                    } else {
                        [0.6, 0.0, 0.0, 1.0]
                    }),
                )
                .unwrap();

            for (resource_count, text) in (&resource_count_storage, &mut text_storage).join() {
                let new_text = format!(
                    "{}",
                    resources.get_amount_for_type(&resource_count.resource_type)
                );
                text.set_text(new_text);
            }
        }
    }
}
