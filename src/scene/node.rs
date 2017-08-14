pub use specs::Entity;

pub struct Node {
    pub entity: Option<Entity>,
    pub sub_nodes: Vec<Node>,
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
}