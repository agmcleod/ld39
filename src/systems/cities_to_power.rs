use std::ops::Deref;
use specs::{Read, System, WriteStorage};
use components::{Button, EntityLookup, Input};

pub struct CitiesToPower;

impl <'a>System<'a> for CitiesToPower {
    type SystemData = (
        WriteStorage<'a, Button>,
        Read<'a, EntityLookup>,
        Read<'a, Input>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut button_storage, entity_lookup_storage, input_storage) = data;

        let input = input_storage.deref();

        let button = button_storage.get_mut(*entity_lookup_storage.get("power_additional_city").unwrap()).unwrap();
        if button.clicked(&input) {

        }
    }
}
