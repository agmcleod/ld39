use rodio::Sink;
use specs::{Component, HashMapStorage};

pub struct ClickSound {
    pub sound: Sink,
}

impl Component for ClickSound {
    type Storage = HashMapStorage<ClickSound>;
}