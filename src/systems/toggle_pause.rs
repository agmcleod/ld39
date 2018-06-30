use components::{Button, Color, EntityLookup, Input, Rect, StateChange, Transform};
use entities::create_colored_rect;
use scene::Node;
use specs::{Entities, Join, Read, System, Write, WriteStorage};
use state::play_state::PlayState;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

pub struct TogglePause {
    pub scene: Arc<Mutex<Node>>,
}

impl<'a> System<'a> for TogglePause {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Button>,
        WriteStorage<'a, Color>,
        Write<'a, EntityLookup>,
        Read<'a, Input>,
        WriteStorage<'a, Rect>,
        Write<'a, StateChange>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut button_storage,
            mut color_storage,
            mut entity_lookup_storage,
            input,
            mut rect_storage,
            mut state_change_storage,
            mut transform_storage,
        ) = data;

        let input: &Input = input.deref();
        let mut transition_to_pause = false;
        for (button, transform) in (&mut button_storage, &mut transform_storage).join() {
            if button.name == "menu".to_string() && button.clicked(input) {
                transition_to_pause = true;
                transform.visible = false;
            }
        }

        if transition_to_pause {
            let node = create_colored_rect::create(
                0.0,
                0.0,
                10.0,
                960,
                640,
                [0.0, 0.0, 0.0, 0.8],
                &entities,
                &mut transform_storage,
                &mut color_storage,
                &mut rect_storage,
            );
            let mut lookup: &mut EntityLookup = entity_lookup_storage.deref_mut();
            lookup
                .entities
                .insert("pause_black".to_string(), node.entity.unwrap());
            let mut scene = self.scene.lock().unwrap();
            scene.add(node);
            let state_change: &mut StateChange = state_change_storage.deref_mut();
            state_change.set(PlayState::get_name(), "pause".to_string());
        }
    }
}
