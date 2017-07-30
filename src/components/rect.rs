use specs::{Component, VecStorage};

// component to identify drqwing a rect
pub struct Rect;

impl Component for Rect {
    type Storage = VecStorage<Rect>;
}