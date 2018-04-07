use specs::{Entities, WriteStorage};
use scene::Node;
use components::{Color, Rect, Transform};

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
) -> Node {
    let entity = entities.create();

    transform_storage.insert(entity, Transform::visible(x, y, z, w, h, 0.0, 1.0, 1.0));
    color_storage.insert(entity, Color(color));
    rect_storage.insert(entity, Rect {});

    let node = Node::new(Some(entity), None);
    node
}
