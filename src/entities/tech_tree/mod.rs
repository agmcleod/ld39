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

pub const SIZE: i32 = 32;

/**
 * This builds out the tech tree from a data stand point. It creates the entities to draw stuff on the screen
 * It then creates the hierarchy for dependencies, so we know when something becomes researchable upon its parent being researched.
 */
pub fn build_tech_tree(world: &mut World) -> TechTreeNode {
    let dimensions = renderer::get_dimensions();
    let center_x = (dimensions[0] - 640.0) / 2.0 - 16.0;

    let coal_entity = world.create_entity()
        .with(Upgrade::new(Buff::Coal, 0.0, 0, Status::Researched))
        .with(Transform::visible(center_x, 32.0, 0.0, SIZE, SIZE, 0.0, 1.0, 1.0))
        .with(Rect{})
        .with(ui::TechTreeNode::new("Unlocks ability to farm coal".to_string(), 0))
        .with(Color(get_color_from_status(&Status::Researched)))
        .build();

    let oil_entity = world.create_entity()
        .with(Upgrade::new(Buff::Oil, 60.0, 10, Status::Researchable))
        .with(Transform::visible(center_x, 96.0, 0.0, SIZE, SIZE, 0.0, 1.0, 1.0))
        .with(Rect{})
        .with(ui::TechTreeNode::new("Unlocks ability to farm oil".to_string(), 10))
        .with(Color(get_color_from_status(&Status::Researchable)))
        .build();

    let solar_entity = world.create_entity()
        .with(Upgrade::new(Buff::Solar, 90.0, 30, Status::Locked))
        .with(Transform::visible(center_x, 160.0, 0.0, SIZE, SIZE, 0.0, 1.0, 1.0))
        .with(Rect{})
        .with(ui::TechTreeNode::new("Unlocks ability to farm solar".to_string(), 30))
        .with(Color(get_color_from_status(&Status::Locked)))
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