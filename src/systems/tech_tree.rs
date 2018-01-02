use std::sync::{Arc, Mutex};
use std::ops::Deref;
use cgmath::Vector3;
use specs::{Entity, Entities, Fetch, FetchMut, Join, ReadStorage, WriteStorage, System};
use scene::Node;
use components::{Color, Rect, Text, Input, Transform};
use components::ui;
use systems::logic::get_absolute_pos;
use entities::create_tooltip;

pub struct TechTree {
    scene: Arc<Mutex<Node>>,
    current_tooltip: Option<Entity>,
}

impl TechTree {
    pub fn new(scene: Arc<Mutex<Node>>) -> TechTree {
        TechTree{
            scene,
            current_tooltip: None,
        }
    }
}

impl <'a>System<'a> for TechTree {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Color>,
        Fetch<'a, Input>,
        WriteStorage<'a, Rect>,
        ReadStorage<'a, ui::TechTreeNode>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut color_storage, input_storage, mut rect_storage, tech_tree_node_storage, mut text_storage, mut transform_storage) = data;

        let input: &Input = input_storage.deref();

        let mut scene = self.scene.lock().unwrap();
        let mut tech_tree_node_entity = None;

        for (entity, _, transform) in (&*entities, &tech_tree_node_storage, &transform_storage).join() {
            let absolute_pos = get_absolute_pos(&scene, &entity, &transform_storage);
            let abs_transform = Transform::visible(absolute_pos.x, absolute_pos.y, 0.0, transform.size.x, transform.size.y, transform.rotation, transform.scale.x, transform.scale.y);
            if abs_transform.contains(&input.mouse_pos.0, &input.mouse_pos.1) {
                tech_tree_node_entity = Some(entity.clone());
            }
        }

        if let Some(tech_tree_node_entity) = tech_tree_node_entity {
            if let Some(current_tooltip) = self.current_tooltip {
                // only create if node entity has changed
                if current_tooltip != tech_tree_node_entity {
                    if let Some(container_node) = scene.get_node_for_entity(tech_tree_node_entity) {
                        let tooltip_node = create_tooltip::create(&entities, &mut color_storage, &mut rect_storage, &mut text_storage, &mut transform_storage, 0.0, 0.0);
                        self.current_tooltip = Some(tooltip_node.entity.unwrap().clone());
                        container_node.sub_nodes.push(tooltip_node);
                    }
                }
            }
        } else {
            if let Some(current_tooltip) = self.current_tooltip {
                scene.remove_node_with_entity(current_tooltip);
                self.current_tooltip = None;
            }
        }
    }
}
