use specs::Entity;

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
        } else if node.sub_nodes.len() > 0 {
            if let Some(n) = get_node_for_entity(&mut node.sub_nodes, target_entity) {
                return Some(n)
            }
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
}