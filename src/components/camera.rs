extern crate specs;

use specs::Component;
use cgmath::Matrix4;

pub struct Camera(pub Matrix4<f32>);

impl Component for Camera {
    type Storage = specs::HashMapStorage<Camera>;
}
