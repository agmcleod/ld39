use specs::{Component, HashMapStorage};

pub struct UpgradeCost;

impl Component for UpgradeCost {
    type Storage = HashMapStorage<UpgradeCost>;
}