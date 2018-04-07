use std::ops::{Deref, DerefMut};
use specs::{Entities, Fetch, FetchMut, Join, ReadStorage, System, WriteStorage};
use components::{AnimationSheet, Button, ClickSound, Gatherer, GathererType, Input, ResourceType,
                 SelectedTile, Text, Transform, Wallet};
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
        ReadStorage<'a, SelectedTile>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
        FetchMut<'a, Wallet>,
        ReadStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut animation_sheet_storage,
            mut button_storage,
            mut click_sound_storage,
            entities,
            mut gatherer_storage,
            input_storage,
            selected_tile_storage,
            mut text_storage,
            mut transform_storage,
            mut wallet_storage,
            wallet_ui_storage,
        ) = data;

        let input: &Input = input_storage.deref();
        let click_sound: &mut ClickSound = click_sound_storage.deref_mut();
        let wallet: &mut Wallet = wallet_storage.deref_mut();

        let mut button_pressed = false;
        let mut build_type = None;
        for button in (&mut button_storage).join() {
            if button.name == "build_coal" && button.clicked(&input) {
                button_pressed = true;
                build_type = Some(ResourceType::Coal);
            } else if button.name == "build_oil" && button.clicked(&input) {
                button_pressed = true;
                build_type = Some(ResourceType::Oil);
            } else if button.name == "build_clean" && button.clicked(&input) {
                button_pressed = true;
                build_type = Some(ResourceType::Clean);
            }

            if button_pressed {
                click_sound.play = true;
            }
        }

        let mut create = false;
        let mut selected_tile_x = 0.0;
        let mut selected_tile_y = 0.0;
        // spend the money, and hide selected tile
        if button_pressed {
            for (_, transform) in (&selected_tile_storage, &mut transform_storage).join() {
                let amount = GathererType::get_type_for_resources_type(&build_type.unwrap())
                    .get_build_cost();
                if transform.visible && wallet.spend(amount) {
                    transform.visible = false;
                    create = true;

                    selected_tile_x = transform.get_pos().x;
                    selected_tile_y = transform.get_pos().y;
                    logic::update_text(
                        format!("{}", wallet.money),
                        &mut text_storage,
                        &wallet_ui_storage,
                    );
                }
            }
        }

        if create {
            // create gatherer
            let gatherer = Gatherer::new(&build_type.unwrap());
            let mut anim = AnimationSheet::new(0.5);
            anim.add_animation("default".to_string(), gatherer.gatherer_type.get_frames());
            anim.set_current_animation("default".to_string());
            let gatherer_entity = entities.create();
            gatherer_storage.insert(gatherer_entity, gatherer);
            animation_sheet_storage.insert(gatherer_entity, anim);
            transform_storage.insert(
                gatherer_entity,
                Transform::visible(selected_tile_x, selected_tile_y, 1.0, 64, 64, 0.0, 1.0, 1.0),
            );

            let mut scene = self.scene.lock().unwrap();
            scene.sub_nodes.push(Node::new(Some(gatherer_entity), None));
        }
    }
}
