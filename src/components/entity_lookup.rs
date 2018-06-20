use specs::Entity;
use std::collections::HashMap;

#[derive(Default)]
pub struct EntityLookup {
    pub entities: HashMap<String, Entity>,
}

impl EntityLookup {
    pub fn new() -> EntityLookup {
        EntityLookup {
            entities: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Entity> {
        self.entities.get(key)
    }
}
