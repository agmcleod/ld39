use specs::{Component, VecStorage};

const MAX_POWER: i32 = 10_000;

pub struct PowerBar {
    pub power_left: i32,
    pub power_per_tick: i32,
}

impl PowerBar {
    pub fn new(power_per_tick: i32) -> PowerBar {
        PowerBar {
            power_left: MAX_POWER,
            power_per_tick,
        }
    }

    pub fn get_max_f32() -> f32 {
        MAX_POWER as f32
    }

    pub fn get_max() -> i32 {
        MAX_POWER
    }

    pub fn get_max_width() -> f32 {
        124.0
    }

    pub fn add_power(&mut self, power: i32) {
        self.power_left += power;
        if self.power_left > MAX_POWER {
            self.power_left = MAX_POWER;
        }
    }
}

impl Component for PowerBar {
    type Storage = VecStorage<PowerBar>;
}
