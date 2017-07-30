use specs::{VecStorage, Component};

pub struct Tile;

impl Tile {
    pub fn get_size() -> i32 {
        64
    }
}

impl Component for Tile {
    type Storage = VecStorage<Tile>;
}