use std::collections::HashMap;
use components::tile::TileType;

pub struct ProtectedNodes {
    pub nodes: HashMap<(i32, i32), TileType>,
}
