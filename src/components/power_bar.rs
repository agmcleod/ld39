use specs::{Component, VecStorage};

pub struct PowerBar {
    pub power_left: usize,
}

impl PowerBar {
    pub fn new() -> PowerBar {
        PowerBar{
            power_left: 100,
        }
    }
}

impl Component for PowerBar {
    type Storage = VecStorage<PowerBar>;
}