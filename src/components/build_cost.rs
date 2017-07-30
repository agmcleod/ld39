use specs::{Component, HashMapStorage};

pub struct BuildCost;

impl Component for BuildCost {
    type Storage = HashMapStorage<BuildCost>;
}