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

    pub fn add_power(&mut self, power: usize) {
        self.power_left += power;
        if self.power_left > 100 {
            self.power_left = 100;
        }
    }
}

impl Component for PowerBar {
    type Storage = VecStorage<PowerBar>;
}