use specs::{Component, VecStorage};

const MAX_POWER: usize = 10_000;

pub struct PowerBar {
    pub power_left: usize,
}

impl PowerBar {
    pub fn new() -> PowerBar {
        PowerBar{
            power_left: MAX_POWER,
        }
    }

    pub fn get_max() -> f32 {
        MAX_POWER as f32
    }

    pub fn add_power(&mut self, power: usize) {
        self.power_left += power;
        if self.power_left > MAX_POWER {
            self.power_left = MAX_POWER;
        }
    }
}

impl Component for PowerBar {
    type Storage = VecStorage<PowerBar>;
}