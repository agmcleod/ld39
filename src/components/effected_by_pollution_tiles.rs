use specs::{Component, Entity, HashMapStorage};

pub struct EffectedByPollutionTiles {
    pub tiles: Vec<Entity>,
}

impl EffectedByPollutionTiles {
    pub fn new() -> Self {
        EffectedByPollutionTiles {
            tiles: Vec::with_capacity(8),
        }
    }

    pub fn clear(&mut self) {
        self.tiles.clear();
    }

    pub fn has_entities(&self) -> bool {
        self.tiles.len() > 0
    }
}

impl Component for EffectedByPollutionTiles {
    type Storage = HashMapStorage<Self>;
}
