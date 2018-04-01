use std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};
use specs::{Entity, Entities, Fetch, FetchMut, Join, ReadStorage, WriteStorage, System};
use scene::Node;
use components::{Button, Color, EntityLookup, Rect, Sprite, Text, Input, Transform, Wallet};
use components::ui;
use entities::{create_tooltip, create_text};
use entities::tech_tree::{Upgrade, Status, get_color_from_status};
use storage_types::*;

pub struct TechTree {
    scene: Arc<Mutex<Node>>,
    current_tooltip: Option<Entity>,
    current_tech_tree_node_entity: Option<Entity>,
}

impl TechTree {
    pub fn new(scene: Arc<Mutex<Node>>) -> TechTree {
        TechTree{
            scene,
            current_tooltip: None,
            current_tech_tree_node_entity: None,
        }
    }
}

impl <'a>System<'a> for TechTree {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Button>,
        WriteStorage<'a, Color>,
        Fetch<'a, EntityLookup>,
        Fetch<'a, Input>,
        WriteStorage<'a, Rect>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, ui::TechTreeButton>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Upgrade>,
        FetchMut<'a, Wallet>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            button_storage,
            mut color_storage,
            entity_lookup_storage,
            input_storage,
            mut rect_storage,
            mut sprite_storage,
            tech_tree_node_storage,
            mut text_storage,
            mut transform_storage,
            mut upgrade_storage,
            mut wallet_storage,
        ) = data;

        let input: &Input = input_storage.deref();
        let lookup: &EntityLookup = entity_lookup_storage.deref();

        let mut scene = self.scene.lock().unwrap();
        let mut tech_tree_node_entity = None;
        let mut tooltip_position = [0.0, 0.0];

        for (entity, _, transform) in (&*entities, &tech_tree_node_storage, &transform_storage).join() {
            let absolute_pos = scene.get_absolute_pos(&entity, &transform_storage);
            let abs_transform = Transform::visible(absolute_pos.x, absolute_pos.y, 0.0, transform.size.x, transform.size.y, transform.rotation, transform.scale.x, transform.scale.y);
            if abs_transform.contains(&input.mouse_pos.0, &input.mouse_pos.1) {
                tech_tree_node_entity = Some(entity.clone());
                tooltip_position[0] = transform.get_pos().x;
                tooltip_position[1] = transform.get_pos().y;
            }
        }

        if let Some(tech_tree_node_entity) = tech_tree_node_entity {
            let create_tooltip = if let Some(current_tech_tree_node_entity) = self.current_tech_tree_node_entity {
                current_tech_tree_node_entity != tech_tree_node_entity
            } else {
                true
            };

            if create_tooltip {
                if let Some(container_node) = scene.get_node_for_entity(*lookup.get(&"tech_tree_container".to_string()).unwrap()) {
                    let tech_tree_node_ui = tech_tree_node_storage.get(tech_tree_node_entity).unwrap();
                    let upgrade = upgrade_storage.get(tech_tree_node_entity).unwrap();
                    let mut tooltip_node = create_tooltip::create(
                        &entities,
                        &mut color_storage,
                        &mut rect_storage,
                        &mut text_storage,
                        &mut transform_storage,
                        tooltip_position[0] - 70.0,
                        tooltip_position[1] + 32.0,
                        160,
                        130,
                        tech_tree_node_ui.text.clone()
                    );
                    self.current_tooltip = Some(tooltip_node.entity.unwrap().clone());
                    self.current_tech_tree_node_entity = Some(tech_tree_node_entity.clone());

                    let mut text_storage_type = TextStorage { entities: &entities, color_storage: &mut color_storage, text_storage: &mut text_storage, transform_storage: &mut transform_storage };

                    if upgrade.status == Status::Researched {
                        let text = create_text::create(
                            &mut text_storage_type,
                            "Researched".to_string(),
                            20.0,
                            5.0, 100.0, 0.0,
                            120, 20,
                            Color([0.5, 0.5, 0.5, 1.0])
                        );
                        tooltip_node.sub_nodes.push(Node::new(Some(text), None));
                    } else {
                        let text = create_text::create(
                            &mut text_storage_type,
                            format!("${}", tech_tree_node_ui.cost),
                            20.0,
                            5.0, 100.0, 0.0,
                            70, 20,
                            Color([1.0, 1.0, 0.0, 1.0])
                        );
                        tooltip_node.sub_nodes.push(Node::new(Some(text), None));


                        let time_left = if upgrade.status == Status::Learning {
                            upgrade.time_to_research - upgrade.current_research_progress
                        } else {
                            upgrade.time_to_research
                        };

                        let text = create_text::create(
                            &mut text_storage_type,
                            format!("{} sec", time_left),
                            20.0,
                            100.0, 100.0, 0.0,
                            70, 20,
                            Color([1.0, 1.0, 0.0, 1.0])
                        );
                        tooltip_node.sub_nodes.push(Node::new(Some(text), None));
                    }

                    container_node.sub_nodes.push(tooltip_node);
                }
            } else if input.mouse_pressed {
                let wallet: &mut Wallet = wallet_storage.deref_mut();
                let upgrade = upgrade_storage.get_mut(tech_tree_node_entity).unwrap();
                if upgrade.status == Status::Researchable && wallet.spend(upgrade.cost) {
                    upgrade.start_learning();
                    *color_storage.get_mut(tech_tree_node_entity).unwrap() = Color(get_color_from_status(&upgrade.status));
                }
            }
        } else {
            if let Some(current_tooltip) = self.current_tooltip {
                scene.remove_node_with_entity(current_tooltip);
                self.current_tooltip = None;
                self.current_tech_tree_node_entity = None;
            }
        }
    }
}
