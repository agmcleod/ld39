use specs::{Component, VecStorage};

#[derive(Rand)]
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
        Tile{
            tile_type,
        }
    }

    pub fn get_sprite_frames(tile_type: &TileType) -> [String; 2] {
        match *tile_type {
            TileType::Open => ["tiles.png".to_string(), "tiles_highlight.png".to_string()],
            TileType::EcoSystem => ["swamp.png".to_string(), "swamp_highlight.png".to_string()],
            TileType::River => ["river.png".to_string(), "river_highlight.png".to_string()],
            TileType::City => ["city.png".to_string(), "city_highlight.png".to_string()],
        }
    }

    pub fn get_size() -> f32 {
        64.0
    }
}

impl Component for Tile {
    type Storage = VecStorage<Tile>;
}
