pub use components::upgrade::*;

use cgmath::Vector2;
use components::ui;
use components::{Color, Shape, Sprite, Transform};
use loader;
use renderer;
use scene::Node;
use serde_json::{self, Value};
use specs::{Entity, World};

pub struct TechTreeNode {
    pub entity: Entity,
    pub sub_nodes: Vec<TechTreeNode>,
}

struct LastUpgrade {
    position: Option<Vector2<f32>>,
    status: Status,
    entity: Option<Entity>,
}

pub const SIZE: u16 = 32;
const SIZE_F: f32 = SIZE as f32;
const Y_INCREMENT: f32 = 64.0;

fn create_line(world: &mut World, last_position: Vector2<f32>, x: f32, y: f32, last_status: Status) -> Entity {
    let last_half_x = last_position.x + SIZE_F / 2.0;
    let last_half_y = last_position.y + SIZE_F / 2.0;
    let half_x = x + SIZE_F / 2.0;
    let half_y = y + SIZE_F / 2.0;
    let points = vec![
        Vector2::new(last_half_x, last_half_y),
        Vector2::new(half_x, half_y),
        Vector2::new(half_x + 2.0, half_y),
        Vector2::new(last_half_x + 2.0, last_half_y),
    ];

    let color = if last_status == Status::Researchable || last_status == Status::Locked {
        [0.7, 0.7, 0.7, 0.3]
    } else {
        [0.7, 0.7, 0.7, 1.0]
    };

    world
        .create_entity()
        .with(Shape::new(points, color))
        .with(Transform::visible_identity())
        .build()
}

fn build_entity_nodes(
    world: &mut World,
    container: &mut Node,
    upgrade_lines_lookup: &mut UpgradeLinesLookup,
    width: f32,
    node: Value,
    last_upgrade: LastUpgrade,
) -> TechTreeNode {
    let x = node["x"].as_f64().unwrap() as f32 * width - SIZE_F / 2.0;
    let y = node["y_tier"].as_u64().unwrap() as f32 * Y_INCREMENT + SIZE_F;
    let description = node["description"].as_str().unwrap().to_string();
    let upgrade: Upgrade = serde_json::from_value(node.clone()).unwrap();
    let cost = upgrade.cost;
    let status = upgrade.status.clone();
    let entity = world
        .create_entity()
        .with(upgrade)
        .with(Transform::visible(
            x, y as f32, 1.0, SIZE, SIZE, 0.0, 1.0, 1.0,
        ))
        .with(Color(get_color_from_status(&status)))
        .with(Sprite {
            frame_name: format!("techtree/{}", node["frame_name"].as_str().unwrap()),
        })
        .with(ui::TechTreeButton::new(description, cost))
        .build();

    if let Some(last_position) = last_upgrade.position {
        if let Some(last_entity) = last_upgrade.entity {
            let entity = create_line(world, last_position, x, y, last_upgrade.status.clone());
            if upgrade_lines_lookup.entities.contains_key(&last_entity) {
                upgrade_lines_lookup.entities.get_mut(&last_entity).unwrap().push(entity.clone());
            } else {
                upgrade_lines_lookup.entities.insert(last_entity, vec![entity.clone()]);
            }
            container.add(Node::new(Some(entity), None));
        }
    }

    let mut tech_tree_node = TechTreeNode {
        entity: entity.clone(),
        sub_nodes: Vec::new(),
    };

    container.add(Node::new(Some(entity), None));

    if let Some(children) = node.get("children") {
        for child in children.as_array().unwrap().iter() {
            tech_tree_node.sub_nodes.push(build_entity_nodes(
                world,
                container,
                upgrade_lines_lookup,
                width,
                child.clone(),
                LastUpgrade{ position: Some(Vector2 { x, y }), status, entity: Some(entity) },
            ));
        }
    }

    tech_tree_node
}

/**
 * This builds out the tech tree from a data source. It creates the entities to draw stuff on the screen
 * It then creates the hierarchy for dependencies, so we know when something becomes researchable upon its parent being researched.
 */
pub fn build_tech_tree(world: &mut World, container: &mut Node, upgrade_lines_lookup: &mut UpgradeLinesLookup) -> TechTreeNode {
    let tech_tree_data = loader::read_text_from_file("resources/tech_tree.json").unwrap();
    let tech_tree_data: serde_json::Value = serde_json::from_str(tech_tree_data.as_ref()).unwrap();

    let dimensions = renderer::get_dimensions();
    let width = dimensions[0] - 640.0;

    let node = tech_tree_data;
    let last_upgrade = LastUpgrade{ position: None, status: Status::Researchable, entity: None };
    build_entity_nodes(world, container, upgrade_lines_lookup, width, node, last_upgrade)
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
