use specs::{Component, HashMapStorage};

pub struct CoalCount;

impl Component for CoalCount {
    type Storage = HashMapStorage<CoalCount>;
}