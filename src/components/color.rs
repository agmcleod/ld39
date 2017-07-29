use specs::{Component, VecStorage};

pub struct Color(pub [f32; 4]);

impl Component for Color {
    type Storage = VecStorage<Color>;
}