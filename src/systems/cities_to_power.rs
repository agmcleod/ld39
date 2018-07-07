use components::{Button, CityPowerState, Color, EntityLookup, Input, Node, PowerBar, Rect, Sprite,
                 Transform, CITY_POWER_STATE_COORDS};
use specs::{Entities, Read, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};

use entities::create_power_bar;
use storage_types::PowerBarStorage;

pub struct CitiesToPower;

impl<'a> System<'a> for CitiesToPower {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Button>,
        Write<'a, CityPowerState>,
        WriteStorage<'a, Color>,
        Write<'a, EntityLookup>,
        Read<'a, Input>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, PowerBar>,
        WriteStorage<'a, Rect>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut button_storage,
            mut city_power_state_storage,
            mut color_storage,
            mut entity_lookup_storage,
            input_storage,
            mut node_storage,
            mut power_bar_storage,
            mut rect_storage,
            mut sprite_storage,
            mut transform_storage,
        ) = data;

        let input = input_storage.deref();
        let entity_lookup = entity_lookup_storage.deref_mut();
        let power_additional_city_button_entity;

        // button is rmeoved after all cities are added
        if let Some(entity) = entity_lookup.get("power_additional_city") {
            power_additional_city_button_entity = *entity;
        } else {
            return;
        }
        let button = button_storage
            .get_mut(power_additional_city_button_entity)
            .unwrap();
        if button.clicked(&input) {
            let city_power_state = city_power_state_storage.deref_mut();
            let border_entity =
                city_power_state.border_entities[city_power_state.current_city_count];

            let sprite = sprite_storage.get_mut(border_entity).unwrap();
            sprite.frame_name = "powerbar.png".to_string();

            let (x, y) = CITY_POWER_STATE_COORDS[city_power_state.current_city_count];

            city_power_state.current_city_count += 1;

            let mut storages = PowerBarStorage {
                entities: &entities,
                color_storage: &mut color_storage,
                power_bar_storage: &mut power_bar_storage,
                rect_storage: &mut rect_storage,
                transform_storage: &mut transform_storage,
            };

            let entity = create_power_bar::create(
                &mut storages,
                x + 3.0,
                y + 3.0,
                40 + (20 * (city_power_state.current_city_count as i32 - 1)),
            );

            let side_bar_container = { entity_lookup.get("side_bar_container").unwrap().clone() };
            let node = node_storage.get_mut(side_bar_container).unwrap();
            node.add(entity);

            if city_power_state.current_city_count == 4 {
                entities
                    .delete(power_additional_city_button_entity)
                    .unwrap();
                entity_lookup.entities.remove("power_additional_city");
            }
        }
    }
}
