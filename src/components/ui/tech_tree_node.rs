use specs::{Component, HashMapStorage};

#[derive(Default)]
pub struct TechTreeNode {
    pub text: String,
}

impl TechTreeNode {
    pub fn new(text: String) -> TechTreeNode {
        TechTreeNode{
            text,
        }
    }
}

impl Component for TechTreeNode {
    type Storage = HashMapStorage<TechTreeNode>;
}
