use components::GathererType;
use specs::Entity;
use std::collections::HashMap;

#[derive(Default)]
pub struct GathererPositions {
    pub gatherers: HashMap<(i32, i32), (GathererType, Entity)>,
}

impl GathererPositions {
    pub fn new() -> Self {
        GathererPositions {
            gatherers: HashMap::new(),
        }
    }
}
