use std::ops::{Deref, DerefMut};
use specs::{Entities, Fetch, FetchMut, Join, ReadStorage, WriteStorage, System};
use components::{BuildCost, Button, Color, Gatherer, GathererType, Input, Resources, ResourceCount, ResourceType, Sprite, Text, Transform, Upgrade, UpgradeCost};
use rusttype::{Point, Scale};

pub struct UpgradeResource;

impl<'a> System<'a> for UpgradeResource {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, BuildCost>,
        WriteStorage<'a, Button>,
        WriteStorage<'a, Color>,
        Fetch<'a, Input>,
        FetchMut<'a, Resources>,
        WriteStorage<'a, ResourceCount>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Upgrade>,
        ReadStorage<'a, UpgradeCost>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, build_cost_storage, mut button_storage, mut color_storage, input_storage, mut resources_storage, mut resource_count_storage, mut sprite_storage, mut text_storage, mut transform_storage, mut upgrade_storage, upgrade_cost_storage) = data;

        let input: &Input = input_storage.deref();
        let resources: &mut Resources = resources_storage.deref_mut();

        let mut upgrade_clicked = false;
        for button in (&mut button_storage).join() {
            if button.clicked(&input) {
                upgrade_clicked = true;
            }
        }

        let mut resource_type_changed = false;
        let mut upgrade_cost = 0;

        if upgrade_clicked {
            for (entity, upgrade, button) in (&*entities, &mut upgrade_storage, &mut button_storage).join() {
                if resources.get_resources(upgrade.get_cost()) > 0 {
                    resources.current_type = match resources.current_type {
                        ResourceType::Coal => ResourceType::Oil,
                        ResourceType::Oil => ResourceType::Clean,
                        _ => panic!("Cannot upgrade"),
                    };

                    resource_type_changed = true;

                    if resources.current_type == ResourceType::Clean {
                        entities.delete(entity);
                    } else {
                        upgrade.gatherer_type = GathererType::Clean;
                        upgrade_cost = upgrade.get_cost();
                        button.frames[0] = "plant_button_1.png".to_string();
                        button.frames[1] = "plant_button_2.png".to_string();
                    }
                }
            }
        }

        for (text, _) in (&mut text_storage, &build_cost_storage).join() {
            let new_text = format!("{}", Gatherer::new(&resources.current_type).gatherer_type.get_build_cost());
            if new_text != text.text {
                text.set_text(new_text);
            }
        }

        let mut added_new_resource_ui = false;
        let mut text_scale_to_copy: Option<Scale> = None;
        let mut text_point_to_copy: Option<Point<f32>> = None;
        for (text, _) in (&text_storage, &resource_count_storage).join() {
            if resource_type_changed && !added_new_resource_ui {
                text_scale_to_copy = Some(text.scale.clone());
                text_point_to_copy = Some(text.offset.clone());
                added_new_resource_ui = true;
            }
        }

        if resource_type_changed && added_new_resource_ui {
            let mut y = 460;
            let frame_name = if resources.current_type == ResourceType::Oil {
                "oil.png".to_string()
            } else {
                y = 420;
                "sun.png".to_string()
            };

            println!("{:?} {:?}", text_scale_to_copy, text_point_to_copy);

            let new_resource_count = entities.create();
            resource_count_storage.insert(new_resource_count, ResourceCount{ resource_type: resources.current_type });
            transform_storage.insert(new_resource_count, Transform::new(670, y, 32, 32, 0.0, 1.0, 1.0));
            sprite_storage.insert(new_resource_count, Sprite{ frame_name: frame_name, visible: true });

            let new_resource_count = entities.create();
            resource_count_storage.insert(new_resource_count, ResourceCount{ resource_type: resources.current_type });
            transform_storage.insert(new_resource_count, Transform::new(720, y, 32, 32, 0.0, 1.0, 1.0));
            text_storage.insert(new_resource_count, Text::new_from(text_scale_to_copy.unwrap(), text_point_to_copy.unwrap()));
            color_storage.insert(new_resource_count, Color([0.0, 1.0, 0.0, 1.0]));
        }

        if resource_type_changed {
            for (text, _) in (&mut text_storage, &upgrade_cost_storage).join() {
                let new_text = format!("{}", upgrade_cost);
                if new_text != text.text {
                    text.set_text(new_text);
                }
            }
        }
    }
}