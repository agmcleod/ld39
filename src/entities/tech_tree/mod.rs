pub mod buff;
mod upgrade;
pub use self::upgrade::*;

use specs::{Entity, World};

pub struct TechTreeNode {
    pub entity: Entity,
    pub sub_nodes: Vec<TechTreeNode>,
}

pub fn build_tech_tree(world: &mut World) -> TechTreeNode {
    let coal_entity = world.create_entity()
        .with(Coal::new())
        .build();

    let oil_entity = world.create_entity()
        .with(Oil::new())
        .build();

    let solar_entity = world.create_entity()
        .with(Solar::new())
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