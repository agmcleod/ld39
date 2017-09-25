use specs::{Component, HashMapStorage};

pub struct SelectedTile;

impl Component for SelectedTile {
    type Storage = HashMapStorage<SelectedTile>;
}