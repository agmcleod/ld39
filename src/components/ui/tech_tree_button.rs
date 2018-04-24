use specs::{Component, HashMapStorage};

#[derive(Default)]
pub struct TechTreeButton {
    pub text: String,
    pub cost: i32,
}

impl TechTreeButton {
    pub fn new(text: String, cost: i32) -> TechTreeButton {
        TechTreeButton { text, cost }
    }
}

impl Component for TechTreeButton {
    type Storage = HashMapStorage<TechTreeButton>;
}
