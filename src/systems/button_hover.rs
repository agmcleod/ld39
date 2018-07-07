use cgmath::Vector3;
use components::{Button, EntityLookup, Input, Node, Sprite, Transform};
use specs::{Entities, Entity, Join, Read, System, WriteStorage};
use std::ops::Deref;

pub struct ButtonHover;

impl<'a> System<'a> for ButtonHover {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Button>,
        Read<'a, EntityLookup>,
        Read<'a, Input>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut button_storage, entity_lookup_storage, input_storage, node_storage, transform_storage, mut sprite_storage) =
            data;

        let input: &Input = input_storage.deref();

        let mouse_x = input.mouse_pos.0;
        let mouse_y = input.mouse_pos.1;

        let lookup = entity_lookup_storage.deref();

        let root_entity = lookup.get("root").unwrap();

        let mut button_entities: Vec<(i32, Entity, Vector3<f32>)> = Vec::new();

        for (button, entity, _, _) in (
            &mut button_storage,
            &*entities,
            &mut sprite_storage,
            &transform_storage,
        ).join()
        {
            button.mouse_is_over = false;
            let absolute_pos = Node::get_absolute_pos(root_entity, &entity, &transform_storage, &node_storage);
            button_entities.push((absolute_pos.z as i32, entity.clone(), absolute_pos));
        }

        button_entities.sort_by(|a, b| b.0.cmp(&a.0));

        let mut found_button = false;

        for (_, button_entity, absolute_pos) in button_entities {
            let transform = transform_storage.get(button_entity).unwrap();
            let button = button_storage.get_mut(button_entity).unwrap();
            let sprite = sprite_storage.get_mut(button_entity).unwrap();

            let abs_transform = Transform::visible(
                absolute_pos.x,
                absolute_pos.y,
                0.0,
                transform.size.x,
                transform.size.y,
                transform.rotation,
                transform.scale.x,
                transform.scale.y,
            );
            if !found_button && !button.disabled && abs_transform.contains(&mouse_x, &mouse_y) {
                button.mouse_is_over = true;
                sprite.frame_name = button.get_hover_frame().clone();
                found_button = true;
            } else {
                button.mouse_is_over = false;
                sprite.frame_name = button.get_default_frame().clone();
            }
        }
    }
}
