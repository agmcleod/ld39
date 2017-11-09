use std::collections::HashMap;
use specs::Entity;

pub struct EntityLookup {
    pub entities: HashMap<String, Entity>,
}

impl EntityLookup {
    pub fn new() -> EntityLookup {
        EntityLookup{
            entities: HashMap::new(),
        }
    }
}
