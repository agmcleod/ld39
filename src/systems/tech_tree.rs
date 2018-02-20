use std::sync::{Arc, Mutex};
use std::ops::Deref;
use specs::{Entity, Entities, Fetch, Join, ReadStorage, WriteStorage, System};
use scene::Node;
use components::{Color, EntityLookup, Rect, Text, Input, Transform};
use components::ui;
use entities::create_tooltip;

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
        WriteStorage<'a, Color>,
        Fetch<'a, EntityLookup>,
        Fetch<'a, Input>,
        WriteStorage<'a, Rect>,
        ReadStorage<'a, ui::TechTreeNode>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut color_storage, entity_lookup_storage, input_storage, mut rect_storage, tech_tree_node_storage, mut text_storage, mut transform_storage) = data;

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
                if let Some(container_node) = scene.get_node_for_entity(*lookup.entities.get(&"tech_tree_container".to_string()).unwrap()) {
                    let tech_tree_node_ui = tech_tree_node_storage.get(tech_tree_node_entity).unwrap();
                    let tooltip_node = create_tooltip::create(&entities, &mut color_storage, &mut rect_storage, &mut text_storage, &mut transform_storage, tooltip_position[0] - 70.0, tooltip_position[1] - 40.0, 160, 100, tech_tree_node_ui.text.clone());
                    self.current_tooltip = Some(tooltip_node.entity.unwrap().clone());
                    self.current_tech_tree_node_entity = Some(tech_tree_node_entity.clone());
                    container_node.sub_nodes.push(tooltip_node);
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
