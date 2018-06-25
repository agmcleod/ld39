use specs::{Component, VecStorage};

pub struct FloatingText {
    pub time_passed: f32,
}

impl FloatingText {
    pub fn new() -> Self {
        FloatingText { time_passed: 0.0 }
    }
}

impl Component for FloatingText {
    type Storage = VecStorage<Self>;
}
