use rand::thread_rng;
use specs::{Entities, Join, Read, System, Write, WriteStorage};

use components::{Button, Color, DeltaTime, EntityLookup, Fade, FadeMode, Input, InternalState,
                 MenuScreen, Node, Rect, StateChange, Transform, TransitionToState};
use entities::create_fade;
use state::{menu_state::MenuState, play_state::PlayState};

const DURATION: f32 = 2.0;

pub struct MenuAnimation {
    animating_tick: f32,
}

impl MenuAnimation {
    pub fn new() -> Self {
        MenuAnimation {
            animating_tick: 0.0,
        }
    }
}

impl<'a> System<'a> for MenuAnimation {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Button>,
        WriteStorage<'a, Color>,
        Read<'a, DeltaTime>,
        WriteStorage<'a, Fade>,
        Read<'a, Input>,
        Read<'a, InternalState>,
        Read<'a, EntityLookup>,
        WriteStorage<'a, MenuScreen>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Rect>,
        Write<'a, StateChange>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, TransitionToState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut button_storage,
            mut color_storage,
            delta_time_storage,
            mut fade_storage,
            input_storage,
            internal_state_storage,
            lookup_storage,
            mut menu_screen_storage,
            mut node_storage,
            mut rect_storage,
            mut state_change_storage,
            mut transform_storage,
            mut transition_to_state_storage,
        ) = data;

        let dt = delta_time_storage.dt;

        if *internal_state_storage != InternalState::Game {
            return;
        }

        for button in (&mut button_storage).join() {
            if button.name == "start" && button.clicked(&input_storage) {
                state_change_storage.state = MenuState::get_name();
                state_change_storage.action = "transition".to_string();
                create_fade::create(
                    &entities,
                    &mut color_storage,
                    &mut fade_storage,
                    &mut node_storage,
                    &mut rect_storage,
                    &mut transform_storage,
                    Some(&mut transition_to_state_storage),
                    Some(PlayState::get_name()),
                    &lookup_storage,
                    FadeMode::In,
                    1.0,
                );
            }
        }

        self.animating_tick += dt;

        if self.animating_tick >= 8.0 {
            self.animating_tick = 0.0;
            for (color, menu_screen, transform) in (
                &mut color_storage,
                &mut menu_screen_storage,
                &mut transform_storage,
            ).join()
            {
                color.0[3] = 1.0;
                menu_screen.animating = false;
                transform.visible = false;
            }
        }

        let mut rng = thread_rng();

        for (color, menu_screen, transform) in (
            &mut color_storage,
            &mut menu_screen_storage,
            &mut transform_storage,
        ).join()
        {
            if !menu_screen.animating && self.animating_tick >= menu_screen.start_time_in_seconds {
                menu_screen.animating = true;
                transform.visible = true;
            } else if menu_screen.animating {
                let diff = (self.animating_tick - menu_screen.start_time_in_seconds) / DURATION;
                if diff < 1.0 {
                    color.0[3] = 1.0 - diff;
                    let start_pos = menu_screen.start_pos;
                    let end_pos = menu_screen.end_pos;

                    transform.set_pos2(
                        (end_pos.0 - start_pos.0) * diff + start_pos.0,
                        (end_pos.1 - start_pos.1) * diff + start_pos.1,
                    );
                // animation is done, hide it
                } else if transform.visible {
                    transform.visible = false;
                    // setup for next run
                    menu_screen.start_pos = MenuScreen::get_random_position(&mut rng);
                    menu_screen.end_pos = MenuScreen::get_random_position(&mut rng);
                    transform.set_pos2(menu_screen.start_pos.0, menu_screen.start_pos.1);
                }
            }
        }
    }
}
