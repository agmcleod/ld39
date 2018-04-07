pub use components::upgrade::*;

use specs::{Entity, World};
use renderer;
use components::{Color, Rect, Transform};
use components::ui;
use loader;
use serde_json::{self, Value};

pub struct TechTreeNode {
    pub entity: Entity,
    pub sub_nodes: Vec<TechTreeNode>,
}

pub const SIZE: u16 = 32;

fn build_entity_nodes(
    world: &mut World,
    width: f32,
    node: Value,
    y: f32,
    y_increment: f32,
) -> TechTreeNode {
    let x = node["x"].as_f64().unwrap() as f32 * width;
    let description = node["description"].as_str().unwrap().to_string();
    let upgrade: Upgrade = serde_json::from_value(node.clone()).unwrap();
    let cost = upgrade.cost;
    let status = upgrade.status.clone();
    let entity = world
        .create_entity()
        .with(upgrade)
        .with(Transform::visible(x, y, 0.0, SIZE, SIZE, 0.0, 1.0, 1.0))
        .with(Rect {})
        .with(Color(get_color_from_status(&status)))
        .with(ui::TechTreeButton::new(description, cost))
        .build();

    let mut tech_tree_node = TechTreeNode {
        entity,
        sub_nodes: Vec::new(),
    };

    if let Some(children) = node.get("children") {
        for child in children.as_array().unwrap().iter() {
            tech_tree_node.sub_nodes.push(build_entity_nodes(
                world,
                width,
                child.clone(),
                y + y_increment,
                y_increment,
            ));
        }
    }

    tech_tree_node
}

/**
 * This builds out the tech tree from a data stand point. It creates the entities to draw stuff on the screen
 * It then creates the hierarchy for dependencies, so we know when something becomes researchable upon its parent being researched.
 */
pub fn build_tech_tree(world: &mut World) -> TechTreeNode {
    let tech_tree_data = loader::read_text_from_file("resources/tech_tree.json").unwrap();
    let tech_tree_data: serde_json::Value = serde_json::from_str(tech_tree_data.as_ref()).unwrap();

    let dimensions = renderer::get_dimensions();
    let width = dimensions[0] - 640.0;
    let center_x = width / 2.0 - (SIZE / 2) as f32;

    let mut node = tech_tree_data;
    let mut y = 32.0;
    let y_increment = y * 2.0;

    build_entity_nodes(world, width, node, y, y_increment)
}

pub fn traverse_tree_mut<F>(node: &mut TechTreeNode, cb: &mut F) -> bool
where
    F: FnMut(&mut TechTreeNode) -> bool,
{
    if cb(node) {
        return true;
    } else {
        for mut sub_node in node.sub_nodes.iter_mut() {
            if traverse_tree_mut(&mut sub_node, cb) {
                return true;
            }
        }
    }

    false
}

pub fn traverse_tree<F>(node: &TechTreeNode, cb: &mut F) -> bool
where
    F: FnMut(&TechTreeNode) -> bool,
{
    if cb(node) {
        return true;
    } else {
        for sub_node in &node.sub_nodes {
            if traverse_tree(&sub_node, cb) {
                return true;
            }
        }
    }

    false
}
