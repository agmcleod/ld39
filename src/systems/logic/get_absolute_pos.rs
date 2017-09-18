use cgmath::Vector2;
use specs::{Entity, ReadStorage};
use scene::Scene;
use scene::node::Node;
use components::Transform;

fn check_node<'a>(node: &Node, entity: &Entity, position: &mut Vector2<f32>, transform_storage: &ReadStorage<'a, Transform>) -> bool {
    let mut found_entity = false;
    if let Some(node_entity) = node.entity {
        let transform = transform_storage.get(node_entity).unwrap();
        position.x += transform.pos.x;
        position.y += transform.pos.y;
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
        }
    }

    found_entity
}

pub fn get_absolute_pos<'a>(scene: &Scene, entity: &Entity, transform_storage: &ReadStorage<'a, Transform>) -> Vector2<f32> {
    let mut position = Vector2{ x: 0.0, y: 0.0 };

    for node in &scene.nodes {
        if check_node(node, entity, &mut position, &transform_storage) {
            break
        }
    }

    position
}