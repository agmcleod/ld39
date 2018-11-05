use specs::{Component, VecStorage};

// Single texture sprite
pub struct Texture {
    pub name: String,
}

impl Texture {
    pub fn new(name: &str) -> Self {
        Texture {
            name: name.to_string(),
        }
    }
}

impl Component for Texture {
    type Storage = VecStorage<Self>;
}
