use components::{Color, Rect, Transform};
use specs::{Entities, LazyUpdate, Entity, Read};

pub fn create(
    x: f32,
    y: f32,
    z: f32,
    w: u16,
    h: u16,
    color: [f32; 4],
    entities: &Entities,
    lazy: &Read<LazyUpdate>,
) -> Entity {
    lazy.create_entity(entities)
        .with(Transform::visible(x, y, z, w, h, 0.0, 1.0, 1.0))
        .with(Color(color))
        .with(Rect{})
        .build()
}
