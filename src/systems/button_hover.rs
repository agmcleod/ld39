use std::ops::Deref;
use specs::{System, Fetch, Join, ReadStorage, WriteStorage};
use components::{Button, Input, Transform, Sprite};

pub struct ButtonHover;

impl<'a> System<'a> for ButtonHover {
    type SystemData = (
        WriteStorage<'a, Button>,
        Fetch<'a, Input>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut button_storage, input_storage, transform_storage, mut sprite_storage) = data;

        let input: &Input = input_storage.deref();

        let mouse_x = input.mouse_pos.0;
        let mouse_y = 640.0 - input.mouse_pos.1;

        for (button, sprite, transform) in (&mut button_storage, &mut sprite_storage, &transform_storage).join() {
            if transform.contains(&mouse_x, &mouse_y) {
                button.mouse_is_over = true;
                sprite.frame_name = button.get_hover_frame().clone();
            } else {
                button.mouse_is_over = false;
                sprite.frame_name = button.get_default_frame().clone();
            }
        }
    }
}