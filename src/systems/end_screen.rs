use std::ops::{Deref, DerefMut};

use gfx_glyph::HorizontalAlign;
use specs::{Entities, Join, Read, System, Write, WriteStorage};
use state::play_state::PlayState;

use components::{Actions, Button, CityPowerState, Color, EntityLookup, Input, Node, Rect, Sprite,
                 StateChange, Text, Transform};
use entities::{create_colored_rect, create_text};
use renderer;
use storage_types::TextStorage;
use systems::logic;

pub struct EndScreen;

impl<'a> System<'a> for EndScreen {
    type SystemData = (
        Entities<'a>,
        Write<'a, Actions>,
        WriteStorage<'a, Button>,
        Read<'a, CityPowerState>,
        WriteStorage<'a, Color>,
        Write<'a, EntityLookup>,
        Read<'a, Input>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Rect>,
        WriteStorage<'a, Sprite>,
        Write<'a, StateChange>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut actions_storage,
            mut button_storage,
            city_power_state_storage,
            mut color_storage,
            mut entity_lookup_storage,
            input_storage,
            mut node_storage,
            mut rect_storage,
            mut sprite_storage,
            mut state_change_storage,
            mut text_storage,
            mut transform_storage,
        ) = data;

        if actions_storage.action_fired("display_end_screen") {
            actions_storage.remove("display_end_screen".to_string());
            let lookup = entity_lookup_storage.deref_mut();
            let root_node = logic::get_root(&lookup, &mut node_storage);

            let entity = create_colored_rect::create(
                0.0,
                0.0,
                8.0,
                960,
                640,
                [0.0, 0.0, 0.0, 0.8],
                &entities,
                &mut transform_storage,
                &mut color_storage,
                &mut rect_storage,
            );
            lookup.entities.insert("pause_black".to_string(), entity);

            root_node.add(entity);

            {
                let mut text_storage = TextStorage {
                    entities: &entities,
                    color_storage: &mut color_storage,
                    text_storage: &mut text_storage,
                    transform_storage: &mut transform_storage,
                };

                let dim = renderer::get_dimensions();
                let text = create_text::create(
                    &mut text_storage,
                    format!(
                        "You were able to provide power to {} cities",
                        city_power_state_storage.current_city_count
                    ),
                    30.0,
                    dim[0] / 2.0,
                    250.0,
                    10.0,
                    600,
                    100,
                    Color([0.0, 0.6, 0.0, 1.0]),
                    Some(HorizontalAlign::Center),
                );

                root_node.add(text);
            }

            let restart = entities.create();
            sprite_storage
                .insert(
                    restart,
                    Sprite {
                        frame_name: "restart.png".to_string(),
                    },
                )
                .unwrap();
            button_storage
                .insert(
                    restart,
                    Button::new(
                        "restart".to_string(),
                        ["restart.png".to_string(), "restart_hover.png".to_string()],
                    ),
                )
                .unwrap();
            transform_storage
                .insert(
                    restart,
                    Transform::visible(384.0, 520.0, 10.0, 192, 50, 0.0, 1.0, 1.0),
                )
                .unwrap();

            root_node.add(restart);
        }

        for button in (&mut button_storage).join() {
            if button.name == "restart" && button.clicked(&input_storage.deref()) {
                state_change_storage.set(PlayState::get_name(), "restart".to_string());
            }
        }
    }
}
