use specs::{Component, HashMapStorage};

pub struct SellCost;

impl Component for SellCost {
    type Storage = HashMapStorage<SellCost>;
}