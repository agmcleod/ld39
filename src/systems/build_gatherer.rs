use std::ops::{Deref, DerefMut};
use specs::{Entities, Fetch, FetchMut, Join, WriteStorage, System};
use components::{AnimationSheet, Button, ClickSound, Gatherer, GathererType, Input, Resources, SelectedTile, Transform};

pub struct BuildGatherer;

impl<'a> System<'a> for BuildGatherer {
    type SystemData = (
        WriteStorage<'a, AnimationSheet>,
        WriteStorage<'a, Button>,
        Fetch<'a, ClickSound>,
        Entities<'a>,
        WriteStorage<'a, Gatherer>,
        Fetch<'a, Input>,
        FetchMut<'a, Resources>,
        WriteStorage<'a, SelectedTile>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut animation_sheet_storage, mut button_storage, click_sound_storage, entities, mut gatherer_storage, input_storage, mut resources_storage, mut selected_tile_storage, mut transform_storage) = data;

        let resources: &mut Resources = resources_storage.deref_mut();
        let input: &Input = input_storage.deref();
        let click_sound: &ClickSound = click_sound_storage.deref();

        let mut button_pressed = false;
        for button in (&mut button_storage).join() {
            if button.name == "build".to_string() && button.clicked(&input) {
                button_pressed = true;
                click_sound.sound.play();
            }
        }

        let mut create = false;
        let mut selected_tile_x = 0;
        let mut selected_tile_y = 0;
        for (selected_tile, transform) in (&mut selected_tile_storage, &transform_storage).join() {
            let amount = GathererType::get_type_for_resources_type(&resources.get_current_type()).get_build_cost();
            if button_pressed && selected_tile.visible && resources.get_resources_to_buy(amount) > 0 {
                selected_tile.visible = false;
                create = true;

                selected_tile_x = transform.pos.x;
                selected_tile_y = transform.pos.y;
            }
        }

        if create {
            let gatherer = Gatherer::new(&resources.get_current_type());
            let mut anim = AnimationSheet::new(1.0);
            anim.add_animation("default".to_string(), gatherer.gatherer_type.get_frames());
            anim.current_animation = "default".to_string();
            let gatherer_entity = entities.create();
            gatherer_storage.insert(gatherer_entity, gatherer);
            animation_sheet_storage.insert(gatherer_entity, anim);
            transform_storage.insert(gatherer_entity, Transform::new(selected_tile_x, selected_tile_y, 64, 64, 0.0, 1.0, 1.0));
        }
    }
}