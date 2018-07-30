use cgmath::Vector3;
use components::Transform;
use specs::{Component, Entity, VecStorage, World, WriteStorage};
use std::cmp;

#[derive(Default)]
pub struct Node {
    pub entities: Vec<Entity>,
    pub children_dirty: bool,
}

impl Node {
    pub fn new() -> Self {
        Node {
            entities: Vec::new(),
            children_dirty: false,
        }
    }

    fn check_node(
        current_entity: &Entity,
        target_entity: &Entity,
        position: &mut Vector3<f32>,
        node_storage: &WriteStorage<Node>,
        transform_storage: &WriteStorage<Transform>,
    ) -> bool {
        let mut found_entity = false;
        let transform = if let Some(transform) = transform_storage.get(*current_entity) {
            transform
        } else {
            return false;
        };

        if !transform.visible {
            return false;
        }
        // increment it before confirming, so sub nodes consider the parent node position
        position.x += transform.get_pos().x;
        position.y += transform.get_pos().y;
        position.z += transform.get_pos().z;
        if *target_entity == *current_entity {
            found_entity = true;
        }

        if !found_entity {
            if let Some(node) = node_storage.get(*current_entity) {
                for entity in &node.entities {
                    if Node::check_node(
                        entity,
                        target_entity,
                        position,
                        node_storage,
                        transform_storage,
                    ) {
                        return true;
                    }
                }
            }
        }

        if !found_entity {
            let transform = transform_storage.get(*current_entity).unwrap();
            // undo if nothing was found
            position.x -= transform.get_pos().x;
            position.y -= transform.get_pos().y;
            position.z -= transform.get_pos().z;
        }

        found_entity
    }

    // potential optimization is to change this into a quad tree
    pub fn get_absolute_pos(
        current_entity: &Entity,
        target_entity: &Entity,
        transform_storage: &WriteStorage<Transform>,
        node_storage: &WriteStorage<Node>,
    ) -> Vector3<f32> {
        let mut position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let node = node_storage.get(*current_entity).unwrap();
        for entity in &node.entities {
            if Node::check_node(
                entity,
                target_entity,
                &mut position,
                node_storage,
                transform_storage,
            ) {
                break;
            }
        }

        position
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
        self.children_dirty = true;
    }

    pub fn add_many(&mut self, entities: Vec<Entity>) {
        self.entities.extend(entities);
        self.children_dirty = true;
    }

    pub fn sort_children<'a>(
        &mut self,
        world: &World,
        transform_storage: &WriteStorage<'a, Transform>,
    ) {
        let mut removed = false;
        self.entities.retain(|e| {
            if world.is_alive(*e) {
                return true;
            }

            removed = true;
            false
        });

        if removed {
            self.children_dirty = true;
        }

        if self.children_dirty {
            self.entities.sort_by(|entity_a, entity_b| {
                let transform_a = if let Some(t) = transform_storage.get(*entity_a) {
                    t
                } else {
                    return cmp::Ordering::Greater;
                };

                let transform_b = if let Some(t) = transform_storage.get(*entity_b) {
                    t
                } else {
                    return cmp::Ordering::Greater;
                };

                (transform_a.get_pos().z as i32).cmp(&(transform_b.get_pos().z as i32))
            });
            self.children_dirty = false;
        }
    }
}

impl Component for Node {
    type Storage = VecStorage<Self>;
}
