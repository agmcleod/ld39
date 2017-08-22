use std::ops::{Deref, DerefMut};
use specs::{Entities, Fetch, FetchMut, Join, ReadStorage, WriteStorage, System};
use components::{AnimationSheet, Button, ClickSound, Gatherer, GathererType, Input, Resources, ResourceType, SelectedTile, Sprite, Text, Transform, Upgrade, UpgradeCost, WinCount};
use std::sync::{Arc, Mutex};
use scene::Scene;
use scene::node::Node;

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
        FetchMut<'a, Resources>,
        WriteStorage<'a, SelectedTile>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Upgrade>,
        ReadStorage<'a, UpgradeCost>,
        WriteStorage<'a, WinCount>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut animation_sheet_storage, mut button_storage, mut click_sound_storage, entities, mut gatherer_storage, input_storage, mut resources_storage, mut selected_tile_storage, mut sprite_storage, mut text_storage, mut transform_storage, mut upgrade_storage, upgrade_cost_storage, mut win_count_storage) = data;

        let resources: &mut Resources = resources_storage.deref_mut();
        let input: &Input = input_storage.deref();
        let click_sound: &mut ClickSound = click_sound_storage.deref_mut();

        let mut button_pressed = false;
        for button in (&mut button_storage).join() {
            if button.name == "build".to_string() && button.clicked(&input) {
                button_pressed = true;
                click_sound.play = true;
            }
        }

        let mut create = false;
        let mut selected_tile_x = 0;
        let mut selected_tile_y = 0;
        for (selected_tile, transform) in (&mut selected_tile_storage, &transform_storage).join() {
            let amount = GathererType::get_type_for_resources_type(&resources.get_current_type()).get_build_cost();
            if button_pressed && selected_tile.visible && resources.get_resources(amount) > 0 {
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
            anim.set_current_animation("default".to_string());
            let gatherer_entity = entities.create();
            gatherer_storage.insert(gatherer_entity, gatherer);
            animation_sheet_storage.insert(gatherer_entity, anim);
            transform_storage.insert(gatherer_entity, Transform::new(selected_tile_x, selected_tile_y, 0, 64, 64, 0.0, 1.0, 1.0));

            let mut scene = self.scene.lock().unwrap();
            scene.nodes.push(Node::new(Some(gatherer_entity), None));

            if resources.get_current_type() == ResourceType::Clean {
                for (text, win_count) in (&mut text_storage, &mut win_count_storage).join() {
                    win_count.count -= 1;
                    let message = win_count.get_message();
                    text.set_text(message);
                }
            }

            if !self.built_one {
                self.built_one = true;
                let upgrade_button_entity = entities.create();
                button_storage.insert(upgrade_button_entity, Button::new("upgrade".to_string(), ["refinery_button_1.png".to_string(), "refinery_button_2.png".to_string()]));
                transform_storage.insert(upgrade_button_entity, Transform::new(670, 90, 0, 64, 64, 0.0, 1.0, 1.0));
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