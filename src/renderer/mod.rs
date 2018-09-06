use cgmath::{ortho, Matrix4};
use gfx;
mod basic;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::Depth;

pub use self::basic::*;

pub fn get_ortho(w: f32, h: f32) -> Matrix4<f32> {
    let mut m = ortho(0.0, w, h, 0.0, 100.0, 0.0);

    m.z.z *= -1.0;
    m
}

pub fn get_dimensions() -> [f32; 2] {
    [960.0, 640.0]
}
