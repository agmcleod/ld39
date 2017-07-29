use specs::{VecStorage, Component};

pub struct Tile;

impl Component for Tile {
    type Storage = VecStorage<Tile>;
}