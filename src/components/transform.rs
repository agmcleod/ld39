use specs::{Component, VecStorage};
use cgmath::Vector2;

pub struct Transform {
    pub pos: Vector2<i32>,
    pub size: Vector2<u16>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl Transform {
    pub fn new(x: i32, y: i32, w: u16, h: u16, rotation: f32, scale_x: f32, scale_y: f32) -> Transform {
        Transform{
            pos: Vector2{ x: x, y: y},
            size: Vector2{ x: w, y: h },
            rotation: rotation,
            scale: Vector2{ x: scale_x, y: scale_y },
        }
    }
}

impl Component for Transform {
    type Storage = VecStorage<Transform>;
}