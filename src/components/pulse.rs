use specs::{Component, VecStorage};

pub struct Pulse {
    // the rate at which to pulse. 0-1-0 alpha
    pub rate: f32,
    // the current time tracked in seconds
    pub time: f32,
}

impl Pulse {
    pub fn new(rate: f32) -> Self {
        Pulse { rate, time: 0.0 }
    }
}

impl Component for Pulse {
    type Storage = VecStorage<Self>;
}
