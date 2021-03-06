use rand::{Rng, ThreadRng};
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Open,
    EcoSystem,
    River,
    City,
}

pub struct Tile {
    pub tile_type: TileType,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        Tile { tile_type }
    }

    pub fn get_sprite_frames(rng: &mut ThreadRng, tile_type: &TileType) -> [String; 2] {
        match *tile_type {
            TileType::Open => ["tiles.png".to_string(), "tiles_highlight.png".to_string()],
            TileType::EcoSystem => ["swamp.png".to_string(), String::new()],
            TileType::River => {
                let rand = rng.gen_range(0, 3);
                if rand == 0 {
                    ["river.png".to_string(), "river_highlight.png".to_string()]
                } else if rand == 1 {
                    ["river2.png".to_string(), "river2_highlight.png".to_string()]
                } else {
                    ["river3.png".to_string(), "river3_highlight.png".to_string()]
                }
            }
            TileType::City => {
                let rand = rng.gen_range(0, 2);
                if rand == 0 {
                    ["city.png".to_string(), String::new()]
                } else {
                    ["city2.png".to_string(), String::new()]
                }
            }
        }
    }

    pub fn get_size() -> f32 {
        64.0
    }
}

impl Component for Tile {
    type Storage = VecStorage<Tile>;
}
