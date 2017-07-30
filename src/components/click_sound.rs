use specs::{Component, HashMapStorage};

pub struct ClickSound {
    pub play: bool,
}

impl Component for ClickSound {
    type Storage = HashMapStorage<ClickSound>;
}