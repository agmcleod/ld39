use components::{Actions, Button, Color, CurrentState, EntityLookup, Input, InternalState, Node,
                 Rect, StateChange, Transform};
use entities::create_colored_rect;
use glutin::VirtualKeyCode;
use specs::{Entities, Join, Read, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use systems::logic;

pub struct TogglePause;

impl<'a> System<'a> for TogglePause {
    type SystemData = (
        Entities<'a>,
        Write<'a, Actions>,
        WriteStorage<'a, Button>,
        WriteStorage<'a, Color>,
        Read<'a, CurrentState>,
        Write<'a, EntityLookup>,
        Read<'a, Input>,
        Read<'a, InternalState>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Rect>,
        Write<'a, StateChange>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut actions_storage,
            mut button_storage,
            mut color_storage,
            current_state_storage,
            mut entity_lookup_storage,
            input,
            internal_state_storage,
            mut node_storage,
            mut rect_storage,
            mut state_change_storage,
            mut transform_storage,
        ) = data;

        let input: &Input = input.deref();
        let internal_state: &InternalState = internal_state_storage.deref();
        let mut transition_to_pause = false;

        if *internal_state == InternalState::Game {
            for button in (&mut button_storage).join() {
                if button.name == "menu".to_string() && button.clicked(input) {
                    transition_to_pause = true;
                }
            }
        }

        let actions = actions_storage.deref_mut();

        if *internal_state == InternalState::Pause
            && (*input.pressed_keys.get(&VirtualKeyCode::Escape).unwrap()
                || actions.action_fired("resume_game"))
        {
            actions.remove("resume_game".to_string());
            let lookup = entity_lookup_storage.deref_mut();
            {
                let entity = lookup.get("pause_black").unwrap();
                entities.delete(*entity).unwrap();
            }
            lookup.entities.remove("pause_black");
            let state_change: &mut StateChange = state_change_storage.deref_mut();
            state_change.set(current_state_storage.0.clone(), "resume".to_string());
        }

        if transition_to_pause {
            let entity = create_colored_rect::create(
                0.0,
                0.0,
                8.0,
                960,
                640,
                [16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0],
                &entities,
                &mut transform_storage,
                &mut color_storage,
                &mut rect_storage,
            );
            let lookup: &mut EntityLookup = entity_lookup_storage.deref_mut();
            lookup.entities.insert("pause_black".to_string(), entity);
            let root_node = logic::get_root(&lookup, &mut node_storage);
            root_node.add(entity);
            let state_change: &mut StateChange = state_change_storage.deref_mut();
            state_change.set(current_state_storage.0.clone(), "pause".to_string());
        }
    }
}
