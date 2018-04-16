use specs::{Component, VecStorage};

pub struct Sprite {
    pub frame_name: String,
}

impl Clone for Sprite {
    fn clone(&self) -> Self {
        Sprite {
            frame_name: self.frame_name.clone(),
        }
    }
}

impl Component for Sprite {
    type Storage = VecStorage<Sprite>;
}
