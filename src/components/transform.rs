use specs::{Component, VecStorage};
use cgmath::{Vector2, Vector3};

pub struct Transform {
    pos: Vector3<f32>,
    pub size: Vector2<u16>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
    pub visible: bool,
    absolute_pos: Vector3<f32>,
    pub dirty_pos: bool,
}

impl Transform {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        w: u16,
        h: u16,
        rotation: f32,
        scale_x: f32,
        scale_y: f32,
        visible: bool,
    ) -> Transform {
        Transform {
            pos: Vector3 { x, y, z },
            size: Vector2 { x: w, y: h },
            rotation,
            scale: Vector2 {
                x: scale_x,
                y: scale_y,
            },
            visible,
            absolute_pos: Vector3 { x, y, z },
            dirty_pos: true,
        }
    }

    pub fn visible(
        x: f32,
        y: f32,
        z: f32,
        w: u16,
        h: u16,
        rotation: f32,
        scale_x: f32,
        scale_y: f32,
    ) -> Transform {
        Self::new(x, y, z, w, h, rotation, scale_x, scale_y, true)
    }

    pub fn visible_identity() -> Transform {
        Transform::visible(0.0, 0.0, 0.0, 0, 0, 0.0, 1.0, 1.0)
    }

    pub fn contains(&self, x: &f32, y: &f32) -> bool {
        let w = self.size.x as f32;
        let h = self.size.y as f32;
        self.pos.x <= *x && self.pos.x + w >= *x && self.pos.y <= *y && self.pos.y + h >= *y
    }

    pub fn get_pos(&self) -> &Vector3<f32> {
        &self.pos
    }

    pub fn set_pos2(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
        self.dirty_pos = true;
    }

    pub fn get_absolute_pos(&self) -> &Vector3<f32> {
        &self.absolute_pos
    }

    pub fn set_absolute_pos(&mut self, pos: Vector3<f32>) {
        self.absolute_pos = pos;
        self.dirty_pos = false;
    }
}

impl Component for Transform {
    type Storage = VecStorage<Transform>;
}
