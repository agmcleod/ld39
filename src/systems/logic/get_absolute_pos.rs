use cgmath::Vector3;
use specs::{Entity, ReadStorage};
use scene::Scene;
use scene::node::Node;
use components::Transform;

fn check_node<'a>(node: &Node, entity: &Entity, position: &mut Vector3<f32>, transform_storage: &ReadStorage<'a, Transform>) -> bool {
    let mut found_entity = false;
    if let Some(node_entity) = node.entity {
        let transform = transform_storage.get(node_entity).unwrap();
        if !transform.visible {
            return false
        }
        position.x += transform.pos.x;
        position.y += transform.pos.y;
        position.z += transform.pos.z;
        if node_entity == *entity {
            found_entity = true;
        }
    }

    if !found_entity {
        for node in &node.sub_nodes {
            found_entity = check_node(&node, entity, position, transform_storage);
        }
    }

    if !found_entity {
        if let Some(node_entity) = node.entity {
            let transform = transform_storage.get(node_entity).unwrap();
            position.x -= transform.pos.x;
            position.y -= transform.pos.y;
            position.z -= transform.pos.z;
        }
    }

    found_entity
}

// potential optimization is to change this into a quad tree
pub fn get_absolute_pos<'a>(scene: &Scene, entity: &Entity, transform_storage: &ReadStorage<'a, Transform>) -> Vector3<f32> {
    let mut position = Vector3{ x: 0.0, y: 0.0, z: 0.0 };

    for node in &scene.nodes {
        if check_node(node, entity, &mut position, &transform_storage) {
            break
        }
    }

    position
}