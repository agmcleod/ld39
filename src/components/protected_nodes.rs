use components::tile::TileType;
use specs::Entity;
use std::collections::HashMap;

#[derive(Default)]
pub struct ProtectedNodes {
    pub nodes: HashMap<(i32, i32), (TileType, Option<Entity>)>,
}
