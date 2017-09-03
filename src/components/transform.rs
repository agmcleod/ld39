use specs::{Component, VecStorage};
use cgmath::{Vector2, Vector3};

pub struct Transform {
    pub pos: Vector3<f32>,
    pub size: Vector2<u16>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl Transform {
    pub fn new(x: f32, y: f32, z: f32, w: u16, h: u16, rotation: f32, scale_x: f32, scale_y: f32) -> Transform {
        Transform{
            pos: Vector3{ x: x, y: y, z: z},
            size: Vector2{ x: w, y: h },
            rotation: rotation,
            scale: Vector2{ x: scale_x, y: scale_y },
        }
    }

    pub fn contains(&self, x: &f32, y: &f32) -> bool {
        let w = self.size.x as f32;
        let h = self.size.y as f32;
        self.pos.x <= *x && self.pos.x + w >= *x &&
        self.pos.y <= *y && self.pos.y + h >= *y
    }
}

impl Component for Transform {
    type Storage = VecStorage<Transform>;
}