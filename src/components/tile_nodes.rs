use components::tile::TileType;
use specs::Entity;
use std::collections::HashMap;

#[derive(Default)]
pub struct TileNodes {
    pub nodes: HashMap<(i32, i32), (TileType, Option<Entity>)>,
}
