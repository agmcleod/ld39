 pub mod buff;
mod upgrade;
pub use self::upgrade::*;

use specs::{Entity, World};
use renderer;
use components::{Color, Rect, Transform};
use components::ui;

pub struct TechTreeNode {
    pub entity: Entity,
    pub sub_nodes: Vec<TechTreeNode>,
}

pub fn build_tech_tree(world: &mut World) -> TechTreeNode {
    let dimensions = renderer::get_dimensions();
    let center_x = (dimensions[0] - 640.0) / 2.0 - 16.0;

    let coal_entity = world.create_entity()
        .with(Coal::new())
        .with(Transform::visible(center_x, 32.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
        .with(Rect{})
        .with(ui::TechTreeNode::new("Unlocks ability to farm coal".to_string()))
        .with(Color(get_color_from_status(&Coal::new().upgrade.status)))
        .build();

    let oil_entity = world.create_entity()
        .with(Oil::new())
        .with(Transform::visible(center_x, 96.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
        .with(Rect{})
        .with(ui::TechTreeNode::new("Unlocks ability to farm oil".to_string()))
        .with(Color(get_color_from_status(&Oil::new().upgrade.status)))
        .build();

    let solar_entity = world.create_entity()
        .with(Solar::new())
        .with(Transform::visible(center_x, 160.0, 0.0, 32, 32, 0.0, 1.0, 1.0))
        .with(Rect{})
        .with(ui::TechTreeNode::new("Unlocks ability to farm solar".to_string()))
        .with(Color(get_color_from_status(&Solar::new().upgrade.status)))
        .build();

    let solar_node = TechTreeNode{
        entity: solar_entity,
        sub_nodes: Vec::new(),
    };

    let oil_node = TechTreeNode{
        entity: oil_entity,
        sub_nodes: vec![solar_node]
    };

    TechTreeNode{
        entity: coal_entity,
        sub_nodes: vec![oil_node]
    }
}

pub fn traverse_tree<F>(node: &mut TechTreeNode, cb: &mut F) -> bool where F: FnMut(&mut TechTreeNode) -> bool {
    if cb(node) {
        return true
    } else {
        for mut sub_node in node.sub_nodes.iter_mut() {
            if traverse_tree(&mut sub_node, cb) {
                return true
            }
        }
    }

    false
}