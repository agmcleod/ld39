pub mod node;
use self::node::Node;

pub struct Scene{
    pub nodes: Vec<Node>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene{
            nodes: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}