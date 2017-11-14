use std::ops::{Deref, DerefMut};
use specs::{System, Fetch, FetchMut, WriteStorage};
use components::{Button, EntityLookup, Input, StateChange, Transform};
use state::play_state::PlayState;

pub struct ToggleTechTree;

impl ToggleTechTree {
    pub fn new() -> ToggleTechTree {
        ToggleTechTree{}
    }

    fn check_show_tech_button(&mut self, lookup: &EntityLookup, input: &Input, button_storage: &mut WriteStorage<Button>, transform_storage: &mut WriteStorage<Transform>, state_change_res: &mut FetchMut<StateChange>) {
        let button = button_storage.get_mut(*lookup.entities.get("show_button_entity").unwrap()).unwrap();
        if button.clicked(&input) {
            {
                let transform = transform_storage.get_mut(*lookup.entities.get("tech_tree_container").unwrap()).unwrap();
                transform.visible = true;
            }

            {
                let transform = transform_storage.get_mut(*lookup.entities.get("side_bar_container").unwrap()).unwrap();
                transform.visible = false;
            }

            let state_change: &mut StateChange = state_change_res.deref_mut();
            state_change.set(PlayState::get_name(), "tech_tree_pause".to_string());
        }
    }

    fn check_resume_from_upgrades_button(&mut self, lookup: &EntityLookup, input: &Input, button_storage: &mut WriteStorage<Button>, transform_storage: &mut WriteStorage<Transform>, state_change_res: &mut FetchMut<StateChange>) {
        let button = button_storage.get_mut(*lookup.entities.get("resume_from_upgrades").unwrap()).unwrap();
        if button.clicked(&input) {
            {
                let transform = transform_storage.get_mut(*lookup.entities.get("tech_tree_container").unwrap()).unwrap();
                transform.visible = false;
            }

            {
                let transform = transform_storage.get_mut(*lookup.entities.get("side_bar_container").unwrap()).unwrap();
                transform.visible = true;
            }

            let state_change: &mut StateChange = state_change_res.deref_mut();
            state_change.set(PlayState::get_name(), "tech_tree_resume".to_string());
        }
    }
}

impl<'a> System<'a> for ToggleTechTree {
    type SystemData = (
        WriteStorage<'a, Button>,
        Fetch<'a, EntityLookup>,
        Fetch<'a, Input>,
        FetchMut<'a, StateChange>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut button_storage, lookup, input, mut state_change_res, mut transform_storage) = data;

        let lookup: &EntityLookup = lookup.deref();
        let input: &Input = input.deref();
        self.check_show_tech_button(&lookup, &input, &mut button_storage, &mut transform_storage, &mut state_change_res);
        self.check_resume_from_upgrades_button(&lookup, &input, &mut button_storage, &mut transform_storage, &mut state_change_res);
    }
}
