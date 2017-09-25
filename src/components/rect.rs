use specs::{Component, VecStorage};

// component to identify drqwing a rect
pub struct Rect {
    pub visible: bool,
}

impl Rect {
    pub fn new() -> Rect {
        Rect{
            visible: true,
        }
    }

    pub fn new_invisible() -> Rect {
        Rect{
            visible: false,
        }
    }
}

impl Component for Rect {
    type Storage = VecStorage<Rect>;
}