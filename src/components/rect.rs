use specs::{Component, VecStorage};

// component to identify drqwing a rect
pub struct Rect;

impl Rect {
    pub fn new() -> Rect {
        Rect{}
    }
}

impl Component for Rect {
    type Storage = VecStorage<Rect>;
}