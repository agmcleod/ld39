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

    fn remove_from_sub_node(nodes: &mut Vec<Node>, target_entity: Entity) -> bool {
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
            if node.sub_nodes.len() > 0 && Scene::remove_from_sub_node(&mut node.sub_nodes, target_entity) {
                return true
            }
        }

        false
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn remove_node_with_entity(&mut self, target_entity: Entity) {
        Scene::remove_from_sub_node(&mut self.nodes, target_entity);
    }
}