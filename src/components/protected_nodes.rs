use std::collections::HashMap;
use specs::Entity;
use components::tile::TileType;

pub struct ProtectedNodes {
    pub nodes: HashMap<(i32, i32), (TileType, Option<Entity>)>,
}
