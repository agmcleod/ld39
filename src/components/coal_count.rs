use specs::{Component, VecStorage};

pub struct CoalCount;

impl Component for CoalCount {
    type Storage = VecStorage<CoalCount>;
}