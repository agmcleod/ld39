use specs::{Component, HashMapStorage};

#[derive(Default)]
pub struct TechTreeButton {
    pub text: String,
}

impl TechTreeButton {
    pub fn new(text: String) -> TechTreeButton {
        TechTreeButton { text }
    }
}

impl Component for TechTreeButton {
    type Storage = HashMapStorage<TechTreeButton>;
}
