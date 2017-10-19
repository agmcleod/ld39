use std::ops::{Deref, DerefMut};
use specs::{Entities, Fetch, FetchMut, Join, ReadStorage, WriteStorage, System};
use components::{BuildCost, Button, Gatherer, GathererType, Input, Resources, ResourceCount, ResourceType, Sprite, Text, Transform, Upgrade, UpgradeCost};
use systems;

pub struct UpgradeResource;

impl<'a> System<'a> for UpgradeResource {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, BuildCost>,
        WriteStorage<'a, Button>,
        Fetch<'a, Input>,
        FetchMut<'a, Resources>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Upgrade>,
        ReadStorage<'a, UpgradeCost>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, build_cost_storage, mut button_storage, input_storage, mut resources_storage, mut text_storage, mut upgrade_storage, upgrade_cost_storage) = data;

        let input: &Input = input_storage.deref();
        let resources: &mut Resources = resources_storage.deref_mut();

        let mut resource_type_changed = false;
        let mut upgrade_cost = 0;

        for (entity, upgrade, button) in (&*entities, &mut upgrade_storage, &mut button_storage).join() {
            if button.clicked(&input) && resources.withdraw_resources(upgrade.get_cost()) > 0 {
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

        systems::logic::update_text(format!("{}", Gatherer::new(&resources.current_type).gatherer_type.get_build_cost()), &mut text_storage, &build_cost_storage);

        if resource_type_changed {
            systems::logic::update_text(format!("{}", upgrade_cost), &mut text_storage, &upgrade_cost_storage);
        }
    }
}