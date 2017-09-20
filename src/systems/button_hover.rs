use std::ops::Deref;
use std::sync::{Arc, Mutex};
use specs::{Entity, Entities, System, Fetch, Join, ReadStorage, WriteStorage};
use components::{Button, Input, Transform, Sprite};
use scene::Scene;
use systems::logic;

pub struct ButtonHover {
    pub scene: Arc<Mutex<Scene>>,
}

impl<'a> System<'a> for ButtonHover {
    type SystemData = (
        WriteStorage<'a, Button>,
        Entities<'a>,
        Fetch<'a, Input>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut button_storage, entities, input_storage, transform_storage, mut sprite_storage) = data;

        let input: &Input = input_storage.deref();

        let mouse_x = input.mouse_pos.0;
        let mouse_y = 640.0 - input.mouse_pos.1;

        let scene = self.scene.lock().unwrap();

        let mut button_entities: Vec<(i32, Entity)> = Vec::new();

        for (button, entity, _, transform) in (&mut button_storage, &*entities, &mut sprite_storage, &transform_storage).join() {
            button.mouse_is_over = false;
            button_entities.push((transform.pos.z as i32, entity.clone()));
        }

        button_entities.sort_by(|a, b| a.0.cmp(&b.0));

        for (_, button_entity) in button_entities {
            let absolute_pos = logic::get_absolute_pos(&scene, &button_entity, &transform_storage);
            let transform = transform_storage.get(button_entity).unwrap();
            let mut button = button_storage.get_mut(button_entity).unwrap();
            let mut sprite = sprite_storage.get_mut(button_entity).unwrap();

            let abs_transform = Transform::new(absolute_pos.x, absolute_pos.y, 0.0, transform.size.x, transform.size.y, transform.rotation, transform.scale.x, transform.scale.y);
            if abs_transform.contains(&mouse_x, &mouse_y) {
                button.mouse_is_over = true;
                sprite.frame_name = button.get_hover_frame().clone();
                break
            } else {
                button.mouse_is_over = false;
                sprite.frame_name = button.get_default_frame().clone();
            }
        }
    }
}