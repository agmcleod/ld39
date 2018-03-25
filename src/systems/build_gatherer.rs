use std::ops::{Deref, DerefMut};
use specs::{Entities, Fetch, FetchMut, Join, ReadStorage, WriteStorage, System};
use components::{AnimationSheet, Button, ClickSound, Gatherer, GathererType, Input, Rect, Resources, SelectedTile, Sprite, Text, Transform, Wallet};
use components::ui::WalletUI;
use std::sync::{Arc, Mutex};
use scene::Node;
use systems::logic;

pub struct BuildGatherer {
    pub scene: Arc<Mutex<Node>>,
}

impl<'a> System<'a> for BuildGatherer {
    type SystemData = (
        WriteStorage<'a, AnimationSheet>,
        WriteStorage<'a, Button>,
        FetchMut<'a, ClickSound>,
        Entities<'a>,
        WriteStorage<'a, Gatherer>,
        Fetch<'a, Input>,
        WriteStorage<'a, Rect>,
        FetchMut<'a, Resources>,
        ReadStorage<'a, SelectedTile>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
        FetchMut<'a, Wallet>,
        ReadStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut animation_sheet_storage, mut button_storage, mut click_sound_storage, entities, mut gatherer_storage, input_storage, rect_storage, mut resources_storage, selected_tile_storage, sprite_storage, mut text_storage, mut transform_storage, mut wallet_storage, wallet_ui_storage) = data;

        let resources: &mut Resources = resources_storage.deref_mut();
        let input: &Input = input_storage.deref();
        let click_sound: &mut ClickSound = click_sound_storage.deref_mut();
        let wallet: &mut Wallet = wallet_storage.deref_mut();

        let mut button_pressed = false;
        for button in (&mut button_storage).join() {
            if button.name == "build_coal" && button.clicked(&input) {
                button_pressed = true;
                click_sound.play = true;
            }
        }

        let mut create = false;
        let mut selected_tile_x = 0.0;
        let mut selected_tile_y = 0.0;
        // spend the money, and hide selected tile
        for (_, transform) in (&selected_tile_storage, &mut transform_storage).join() {
            // TODO: needs to be updated to build arbitrary type, not just the furthest reached tech level
            let amount = GathererType::get_type_for_resources_type(&resources.get_current_type()).get_build_cost();
            if button_pressed && transform.visible && wallet.spend(amount) {
                transform.visible = false;
                create = true;

                selected_tile_x = transform.get_pos().x;
                selected_tile_y = transform.get_pos().y;
                logic::update_text(format!("{}", wallet.money), &mut text_storage, &wallet_ui_storage);
            }
        }

        if create {
            // create gatherer
            // TODO: needs to be updated to build arbitrary type
            let gatherer = Gatherer::new(&resources.get_current_type());
            let mut anim = AnimationSheet::new(0.5);
            anim.add_animation("default".to_string(), gatherer.gatherer_type.get_frames());
            anim.set_current_animation("default".to_string());
            let gatherer_entity = entities.create();
            gatherer_storage.insert(gatherer_entity, gatherer);
            animation_sheet_storage.insert(gatherer_entity, anim);
            transform_storage.insert(gatherer_entity, Transform::visible(selected_tile_x, selected_tile_y, 1.0, 64, 64, 0.0, 1.0, 1.0));

            let mut scene = self.scene.lock().unwrap();
            scene.sub_nodes.push(Node::new(Some(gatherer_entity), None));
        }
    }
}