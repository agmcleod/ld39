use specs::{Component, HashMapStorage};

pub struct WinCount{
    pub count: usize,
}

impl Component for WinCount {
    type Storage = HashMapStorage<WinCount>;
}