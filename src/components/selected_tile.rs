use specs::{Component, HashMapStorage};

pub struct SelectedTile{
    pub visible: bool,
}

impl Component for SelectedTile {
    type Storage = HashMapStorage<SelectedTile>;
}