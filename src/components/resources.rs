use specs::{Component, HashMapStorage};

pub struct Resources {
    pub coal: usize,
    pub oil: usize,
    pub power: usize,
}

impl Resources {
    pub fn new() -> Resources {
        Resources{
            coal: 50,
            oil: 0,
            power: 0,
        }
    }
}

impl Component for Resources {
    type Storage = HashMapStorage<Resources>;
}