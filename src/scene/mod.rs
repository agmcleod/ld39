use specs::{Entity, WriteStorage};
use cgmath::Vector3;
use components::Transform;

pub struct Node {
    pub entity: Option<Entity>,
    pub sub_nodes: Vec<Node>,
}

fn get_node_for_entity(nodes: &mut Vec<Node>, target_entity: Entity) -> Option<&mut Node> {
    for node in nodes.iter_mut() {
        if let Some(e) = node.entity {
            if e == target_entity {
                return Some(node);
            }
        }
        if let Some(n) = get_node_for_entity(&mut node.sub_nodes, target_entity) {
            return Some(n)
        }
    }

    None
}

fn remove_from_sub_nodes(nodes: &mut Vec<Node>, target_entity: Entity) -> bool {
    let to_remove = nodes.iter().position(|node| {
        if let Some(entity) = node.entity {
            if entity == target_entity {
                return true
            }
        }
        false
    });

    if let Some(to_remove) = to_remove {
        nodes.remove(to_remove);
        return true
    }

    for node in nodes.iter_mut() {
        if node.sub_nodes.len() > 0 && remove_from_sub_nodes(&mut node.sub_nodes, target_entity) {
            return true
        }
    }

    false
}

impl Node {
    pub fn new(entity: Option<Entity>, nodes: Option<Vec<Node>>) -> Node {
        let sub_nodes = match nodes {
            Some(n) => n,
            None => vec![],
        };

        Node{
            entity: entity,
            sub_nodes: sub_nodes,
        }
    }

    pub fn clear(&mut self) {
        self.sub_nodes.clear();
    }

    pub fn get_node_for_entity(&mut self, target_entity: Entity) -> Option<&mut Node> {
        if let Some(e) = self.entity {
            if e == target_entity {
                return Some(self);
            }
        }

        get_node_for_entity(&mut self.sub_nodes, target_entity)
    }

    pub fn remove_node_with_entity(&mut self, target_entity: Entity) {
        remove_from_sub_nodes(&mut self.sub_nodes, target_entity);
    }

    pub fn add(&mut self, node: Node) {
        self.sub_nodes.push(node);
    }

    fn check_node<'a>(&self, node: &Node, entity: &Entity, position: &mut Vector3<f32>, transform_storage: &WriteStorage<'a, Transform>) -> bool {
        let mut found_entity = false;
        if let Some(node_entity) = node.entity {
            let transform = transform_storage.get(node_entity).unwrap();
            if !transform.visible {
                return false
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
                    return true
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
    pub fn get_absolute_pos<'a>(&self, entity: &Entity, transform_storage: &WriteStorage<'a, Transform>) -> Vector3<f32> {
        let mut position = Vector3{ x: 0.0, y: 0.0, z: 0.0 };

        for node in &self.sub_nodes {
            if self.check_node(node, entity, &mut position, &transform_storage) {
                break
            }
        }

        position
    }
}