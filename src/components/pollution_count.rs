use specs::{Component, HashMapStorage};

pub struct PollutionCount {
    pub count: i32,
}

impl Component for PollutionCount {
    type Storage = HashMapStorage<Self>;
}
