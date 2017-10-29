use specs::{VecStorage, Component};

pub struct Sprite {
    pub frame_name: String,
}

impl Component for Sprite {
    type Storage = VecStorage<Sprite>;
}