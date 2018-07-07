use components::{Color, Rect, Transform};
use specs::{Entity, Entities, WriteStorage};

pub fn create(
    x: f32,
    y: f32,
    z: f32,
    w: u16,
    h: u16,
    color: [f32; 4],
    entities: &Entities,
    transform_storage: &mut WriteStorage<Transform>,
    color_storage: &mut WriteStorage<Color>,
    rect_storage: &mut WriteStorage<Rect>,
) -> Entity {
    let entity = entities.create();

    transform_storage
        .insert(entity, Transform::visible(x, y, z, w, h, 0.0, 1.0, 1.0))
        .unwrap();
    color_storage.insert(entity, Color(color)).unwrap();
    rect_storage.insert(entity, Rect {}).unwrap();

    entity
}
