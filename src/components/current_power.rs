use specs::{Component, HashMapStorage};

// identifier for the bar filling up
pub struct CurrentPower;

impl CurrentPower {
    pub fn get_max_with() -> u16 {
        252
    }
}

impl Component for CurrentPower {
    type Storage = HashMapStorage<CurrentPower>;
}