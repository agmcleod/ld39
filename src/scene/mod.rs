pub mod node;
use self::node::Node;
use specs::Entity;

pub struct Scene{
    pub nodes: Vec<Node>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene{
            nodes: vec![],
        }
    }

    fn remove_from_sub_node(node: &mut Node, target_entity: Entity) -> bool {
        let to_remove = node.sub_nodes.iter().position(|node| {
            if let Some(entity) = node.entity {
                if entity == target_entity {
                    return true
                }
            }
            false
        });

        if let Some(to_remove) = to_remove {
            node.sub_nodes.remove(to_remove);
            return true
        }

        for node in &mut node.sub_nodes {
            if Scene::remove_from_sub_node(node, target_entity) {
                return true
            }
        }

        false
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn remove_node_with_entity(&mut self, target_entity: Entity) {
        let to_remove = self.nodes.iter().position(|node| {
            if let Some(node_entity) = node.entity {
                if node_entity == target_entity {
                    return true
                }
            }
            false
        });

        if let Some(to_remove) = to_remove {
            self.nodes.remove(to_remove);
            return
        }

        for node in &mut self.nodes {
            if node.sub_nodes.len() > 0 {
                Scene::remove_from_sub_node(node, target_entity);
            }
        }
    }
}