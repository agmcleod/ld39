use specs::{Component, HashMapStorage};

#[derive(Default)]
pub struct TechTreeButton {
    pub text: String,
    pub cost: usize,
}

impl TechTreeButton {
    pub fn new(text: String, cost: usize) -> TechTreeButton {
        TechTreeButton { text, cost }
    }
}

impl Component for TechTreeButton {
    type Storage = HashMapStorage<TechTreeButton>;
}
