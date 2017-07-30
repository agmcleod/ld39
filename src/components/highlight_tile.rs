use specs::{Component, HashMapStorage};

pub struct HighlightTile{
    pub visible: bool,
}

impl Component for HighlightTile {
    type Storage = HashMapStorage<HighlightTile>;
}