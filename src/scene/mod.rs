use specs::{Entities, Entity, WriteStorage};
use cgmath::Vector3;
use components::Transform;

pub struct Node {
    pub entity: Option<Entity>,
    pub sub_nodes: Vec<Node>,
    children_dirty: bool,
}

impl Node {
    pub fn new(entity: Option<Entity>, nodes: Option<Vec<Node>>) -> Node {
        let sub_nodes = match nodes {
            Some(n) => n,
            None => vec![],
        };

        Node {
            entity: entity,
            sub_nodes: sub_nodes,
            children_dirty: true,
        }
    }

    pub fn clear(&mut self) {
        self.sub_nodes.clear();
    }

    pub fn get_sub_nodes(&self) -> &Vec<Node> {
        &self.sub_nodes
    }

    pub fn get_node_for_entity(&mut self, target_entity: Entity) -> Option<&mut Node> {
        if let Some(e) = self.entity {
            if e == target_entity {
                return Some(self);
            }
        }

        get_node_for_entity(&mut self.sub_nodes, target_entity)
    }

    pub fn remove_by_index(&mut self, index: usize) {
        self.sub_nodes.remove(index);
    }

    pub fn remove_node_with_entity(&mut self, entities: &Entities, target_entity: Entity) {
        // calls a separate function to avoid recursive mutable borrows
        remove_from_sub_nodes(&mut self.sub_nodes, entities, target_entity, false);
        self.children_dirty = true;
    }

    pub fn add(&mut self, node: Node) {
        self.sub_nodes.push(node);
        self.children_dirty = true;
    }

    pub fn add_many(&mut self, nodes: Vec<Node>) {
        self.sub_nodes.extend(nodes);
        self.children_dirty = true;
    }

    pub fn sort_children<'a>(&mut self, transform_storage: &WriteStorage<'a, Transform>) {
        if self.children_dirty {
            self.sub_nodes.sort_by(|node_a, node_b| {
                let transform_a = transform_storage.get(node_a.entity.unwrap()).unwrap();
                let transform_b = transform_storage.get(node_b.entity.unwrap()).unwrap();

                (transform_a.get_pos().z as i32).cmp(&(transform_b.get_pos().z as i32))
            });
            self.children_dirty = false;
        }
    }

    fn check_node<'a>(
        &self,
        node: &Node,
        entity: &Entity,
        position: &mut Vector3<f32>,
        transform_storage: &WriteStorage<'a, Transform>,
    ) -> bool {
        let mut found_entity = false;
        if let Some(node_entity) = node.entity {
            let transform = transform_storage.get(node_entity).unwrap();
            if !transform.visible {
                return false;
            }
            // increment it before confirming, so sub nodes consider the parent node position
            position.x += transform.get_pos().x;
            position.y += transform.get_pos().y;
            position.z += transform.get_pos().z;
            if node_entity == *entity {
                found_entity = true;
            }
        }

        if !found_entity {
            for node in &node.sub_nodes {
                if self.check_node(&node, entity, position, transform_storage) {
                    return true;
                }
            }
        }

        if !found_entity {
            if let Some(node_entity) = node.entity {
                let transform = transform_storage.get(node_entity).unwrap();
                // undo if nothing was found
                position.x -= transform.get_pos().x;
                position.y -= transform.get_pos().y;
                position.z -= transform.get_pos().z;
            }
        }

        found_entity
    }

    // potential optimization is to change this into a quad tree
    pub fn get_absolute_pos<'a>(
        &self,
        entity: &Entity,
        transform_storage: &WriteStorage<'a, Transform>,
    ) -> Vector3<f32> {
        let mut position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        for node in &self.sub_nodes {
            if self.check_node(node, entity, &mut position, &transform_storage) {
                break;
            }
        }

        position
    }
}

fn get_node_for_entity(nodes: &mut Vec<Node>, target_entity: Entity) -> Option<&mut Node> {
    for node in nodes.iter_mut() {
        if let Some(e) = node.entity {
            if e == target_entity {
                return Some(node);
            }
        }
        if let Some(n) = get_node_for_entity(&mut node.sub_nodes, target_entity) {
            return Some(n);
        }
    }

    None
}

fn remove_from_sub_nodes(
    nodes: &mut Vec<Node>,
    entities: &Entities,
    target_entity: Entity,
    delete_all_entities_found: bool,
) -> bool {
    let to_remove = nodes.iter().position(|node| {
        if let Some(entity) = node.entity {
            if delete_all_entities_found {
                entities.delete(entity).unwrap();
            }
            if entity == target_entity {
                return true;
            }
        }
        false
    });

    if let Some(to_remove) = to_remove {
        entities.delete(target_entity).unwrap();
        // since we found the node, we need to remove all sub node's entities.
        // we keep the target the same, so no other node is found to be the same one.
        // no entity should be in the scene graph more than once
        remove_from_sub_nodes(
            &mut nodes[to_remove].sub_nodes,
            entities,
            target_entity,
            true,
        );
        nodes.remove(to_remove);
        return true;
    }

    for node in nodes.iter_mut() {
        if node.sub_nodes.len() > 0
            && remove_from_sub_nodes(
                &mut node.sub_nodes,
                entities,
                target_entity,
                delete_all_entities_found,
            ) {
            return true;
        }
    }

    false
}
