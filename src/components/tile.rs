use specs::{VecStorage, Component};

pub struct Tile;

impl Tile {
    pub fn get_size() -> f32 {
        64.0
    }
}

impl Component for Tile {
    type Storage = VecStorage<Tile>;
}