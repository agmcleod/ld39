use specs::{Component, HashMapStorage};

#[derive(Default)]
pub struct TechTreeNode {
    pub text: String,
    pub cost: usize,
}

impl TechTreeNode {
    pub fn new(text: String, cost: usize) -> TechTreeNode {
        TechTreeNode{
            text,
            cost,
        }
    }
}

impl Component for TechTreeNode {
    type Storage = HashMapStorage<TechTreeNode>;
}
