use specs::{Component, HashMapStorage};
use components::GathererType;

pub struct Upgrade {
    pub gatherer_type: GathererType,
}

impl Upgrade {
    pub fn new() -> Upgrade {
        Upgrade{
            gatherer_type: GathererType::Oil,
        }
    }

    pub fn get_cost(&self) -> usize {
        match self.gatherer_type {
            GathererType::Oil => 50,
            GathererType::Clean => 100,
            _ => 0,
        }
    }
}

impl Component for Upgrade {
    type Storage = HashMapStorage<Upgrade>;
}