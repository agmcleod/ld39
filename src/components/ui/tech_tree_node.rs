use specs::{Component, NullStorage};

#[derive(Default)]
pub struct TechTreeNode;

impl Component for TechTreeNode {
    type Storage = NullStorage<TechTreeNode>;
}
