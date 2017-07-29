use specs::{Component, VecStorage};

pub struct Power {
    pub power_left: usize,
}

impl Power {
    pub fn new() -> Power {
        Power{
            power_left: 100,
        }
    }
}

impl Component for Power {
    type Storage = VecStorage<Power>;
}