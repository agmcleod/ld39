use std::ops::Deref;
use specs::{System, Fetch, WriteStorage};
use components::{Button, EntityLookup, Input, Transform};

pub struct ToggleTechTree;

impl ToggleTechTree {
    pub fn new() -> ToggleTechTree {
        ToggleTechTree{}
    }
}

impl<'a> System<'a> for ToggleTechTree {
    type SystemData = (
        WriteStorage<'a, Button>,
        Fetch<'a, EntityLookup>,
        Fetch<'a, Input>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut button_storage, lookup, input, mut transform_storage) = data;

        let lookup: &EntityLookup = lookup.deref();
        let input: &Input = input.deref();
        let button = button_storage.get_mut(*lookup.entities.get("show_button_entity").unwrap()).unwrap();
        if button.clicked(&input) {
            let transform = transform_storage.get_mut(*lookup.entities.get("tech_tree_container").unwrap()).unwrap();
            transform.visible = true;
        }
    }
}
