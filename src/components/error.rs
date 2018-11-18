use specs::{Component, HashMapStorage};

pub struct Error {
    pub tick: f32,
}

impl Error {
    pub fn new(tick: f32) -> Self {
        Error { tick }
    }
}

impl Component for Error {
    type Storage = HashMapStorage<Self>;
}
