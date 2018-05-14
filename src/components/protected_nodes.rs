use std::collections::HashMap;
use specs::Entity;
use components::tile::TileType;

#[derive(Default)]
pub struct ProtectedNodes {
    pub nodes: HashMap<(i32, i32), (TileType, Option<Entity>)>,
}
