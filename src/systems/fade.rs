use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

use components::{self, Color, DeltaTime, FadeMode, StateChange, TransitionToState};

pub struct Fade;

impl<'a> System<'a> for Fade {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Color>,
        Read<'a, DeltaTime>,
        WriteStorage<'a, components::Fade>,
        Write<'a, StateChange>,
        ReadStorage<'a, TransitionToState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut color_storage,
            delta_time_storage,
            mut fade_storage,
            mut state_change_storage,
            transition_to_state_storage,
        ) = data;

        for (entity, color, fade) in (&*entities, &mut color_storage, &mut fade_storage).join() {
            fade.tick -= delta_time_storage.dt;
            let mut alpha = fade.tick / fade.start_tick;
            if fade.mode == FadeMode::In {
                alpha = (1.0 - alpha).abs();
            }

            color.0[3] = alpha;
            if fade.tick <= 0.0 {
                if let Some(transition_to_state) = transition_to_state_storage.get(entity) {
                    state_change_storage.action = "start".to_string();
                    state_change_storage.state = transition_to_state.state.clone();
                } else {
                    // if we dont transition, clean it up manually
                    entities.delete(entity).unwrap();
                }
            }
        }
    }
}
