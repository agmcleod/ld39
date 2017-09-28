use std::ops::{Deref, DerefMut};
use specs::{Entities, Fetch, FetchMut, Join, ReadStorage, WriteStorage, System};
use components::{AnimationSheet, Button, ClickSound, Gatherer, GathererType, Input, Rect, Resources, ResourceType, SelectedTile, Sprite, Text, Transform, Upgrade, UpgradeCost, Wallet, WalletUI, WinCount};
use std::sync::{Arc, Mutex};
use scene::Scene;
use scene::node::Node;
use systems::logic;

pub struct BuildGatherer {
    pub built_one: bool,
    pub scene: Arc<Mutex<Scene>>,
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
        WriteStorage<'a, Upgrade>,
        ReadStorage<'a, UpgradeCost>,
        FetchMut<'a, Wallet>,
        ReadStorage<'a, WalletUI>,
        WriteStorage<'a, WinCount>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut animation_sheet_storage, mut button_storage, mut click_sound_storage, entities, mut gatherer_storage, input_storage, mut rect_storage, mut resources_storage, mut selected_tile_storage, mut sprite_storage, mut text_storage, mut transform_storage, mut upgrade_storage, upgrade_cost_storage, mut wallet_storage, wallet_ui_storage, mut win_count_storage) = data;

        let resources: &mut Resources = resources_storage.deref_mut();
        let input: &Input = input_storage.deref();
        let click_sound: &mut ClickSound = click_sound_storage.deref_mut();
        let wallet: &mut Wallet = wallet_storage.deref_mut();

        let mut button_pressed = false;
        for button in (&mut button_storage).join() {
            if button.name == "build-coal" && button.clicked(&input) {
                button_pressed = true;
                click_sound.play = true;
            }
        }

        let mut create = false;
        let mut selected_tile_x = 0.0;
        let mut selected_tile_y = 0.0;
        // spend the money, and hide selected tile
        for (_, rect, transform) in (&selected_tile_storage, &mut rect_storage, &transform_storage).join() {
            let amount = GathererType::get_type_for_resources_type(&resources.get_current_type()).get_build_cost();
            if button_pressed && rect.visible && wallet.spend(amount) {
                rect.visible = false;
                create = true;

                selected_tile_x = transform.pos.x;
                selected_tile_y = transform.pos.y;
                logic::update_text(format!("{}", wallet.money), &mut text_storage, &wallet_ui_storage);
            }
        }

        if create {
            // create gatherer
            let gatherer = Gatherer::new(&resources.get_current_type());
            let mut anim = AnimationSheet::new(1.0);
            anim.add_animation("default".to_string(), gatherer.gatherer_type.get_frames());
            anim.set_current_animation("default".to_string());
            let gatherer_entity = entities.create();
            gatherer_storage.insert(gatherer_entity, gatherer);
            animation_sheet_storage.insert(gatherer_entity, anim);
            transform_storage.insert(gatherer_entity, Transform::new(selected_tile_x, selected_tile_y, 0.0, 64, 64, 0.0, 1.0, 1.0));

            let mut scene = self.scene.lock().unwrap();
            scene.nodes.push(Node::new(Some(gatherer_entity), None));

            // update win condition
            if resources.get_current_type() == ResourceType::Clean {
                for (text, win_count) in (&mut text_storage, &mut win_count_storage).join() {
                    win_count.count -= 1;
                    let message = win_count.get_message();
                    text.set_text(message);
                }
            }

            // create upgrade button
            if !self.built_one {
                self.built_one = true;
                let upgrade_button_entity = entities.create();
                button_storage.insert(upgrade_button_entity, Button::new("upgrade".to_string(), ["refinery_button_1.png".to_string(), "refinery_button_2.png".to_string()]));
                transform_storage.insert(upgrade_button_entity, Transform::new(670.0, 486.0, 0.0, 64, 64, 0.0, 1.0, 1.0));
                upgrade_storage.insert(upgrade_button_entity, Upgrade::new());
                sprite_storage.insert(upgrade_button_entity, Sprite{ frame_name: "refinery_1.png".to_string(), visible: true });

                scene.nodes.push(Node::new(Some(upgrade_button_entity), None));

                for (_, text) in (&upgrade_cost_storage, &mut text_storage).join() {
                    text.visible = true;
                }
            }
        }
    }
}