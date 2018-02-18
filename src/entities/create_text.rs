use specs::{EntityBuilder, World};
use cgmath::Vector3;
use components::{Color, Text, Transform};

pub fn create(world: &mut World, text: String, size: f32, x: f32, y: f32, z: f32, w: u16, h: u16, color: Color) -> EntityBuilder {
    let pos = Vector3{ x, y, z };
    let text = Text::new_with_absolute_position(size, w, h, pos.clone(), text);

    world.create_entity()
        .with(Transform::visible(pos.x, pos.y, pos.z, w, h, 0.0, 1.0, 1.0))
        .with(text)
        .with(color)
}