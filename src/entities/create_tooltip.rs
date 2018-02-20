use specs::{Entities, WriteStorage};
use cgmath::{Vector3, Zero};
use scene::Node;
use components::{Color, Rect, Text, Transform};

pub fn create(
    entities: &Entities,
    color_storage: &mut WriteStorage<Color>,
    rect_storage: &mut WriteStorage<Rect>,
    text_storage: &mut WriteStorage<Text>,
    transform_storage: &mut WriteStorage<Transform>,
    x: f32,
    y: f32,
    w: u16,
    h: u16,
    text: String,
) -> Node {

    let background = entities.create();
    transform_storage.insert(background.clone(), Transform::visible(0.0, 0.0, 0.0, w, h, 0.0, 1.0, 1.0));
    color_storage.insert(background.clone(), Color([0.0, 0.0, 0.0, 0.6]));
    rect_storage.insert(background.clone(), Rect{});

    let tooltip_container = entities.create();
    transform_storage.insert(tooltip_container.clone(), Transform::visible(x, y, 50.0, w, h, 0.0, 1.0, 1.0));
    let mut container_node = Node::new(Some(tooltip_container.clone()), Some(vec![
        Node::new(Some(background), None)
    ]));

    let text_entity = entities.create();
    text_storage.insert(text_entity.clone(), Text::new_with_text(20.0, w, h, text));
    transform_storage.insert(text_entity.clone(), Transform::visible(5.0, 5.0, 0.0, w - 5, h - 5, 0.0, 1.0, 1.0));
    color_storage.insert(text_entity.clone(), Color([1.0, 1.0, 1.0, 1.0]));

    container_node.add(Node::new(Some(text_entity.clone()), None));

    container_node
}