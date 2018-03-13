use specs::{Entity, Entities, EntityBuilder, WriteStorage, World};
use cgmath::Vector3;
use components::{Color, Text, Transform};

pub fn create<'a>(entities: &Entities, color_storage: &mut WriteStorage<'a, Color>, text_storage: &mut WriteStorage<'a, Text>, transform_storage: &mut WriteStorage<'a, Transform>, text: String, size: f32, x: f32, y: f32, z: f32, w: u16, h: u16, color: Color) -> Entity {
    let text = Text::new_with_text(size, w, h, text);

    let entity = entities.create();
    transform_storage.insert(entity, Transform::visible(x, y, z, w, h, 0.0, 1.0, 1.0));
    text_storage.insert(entity, text);
    color_storage.insert(entity, color);

    entity
}