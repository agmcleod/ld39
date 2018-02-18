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
    text: String,
) -> Node {
    // w && h fixed for now
    let w = 150;
    let h = 75;

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
    text_storage.insert(text_entity.clone(), Text::new_with_absolute_position(20.0, w, h, Vector3::<f32>::zero(), text));
    transform_storage.insert(text_entity.clone(), Transform::visible(0.0, 0.0, 0.0, w, h, 0.0, 1.0, 1.0));
    color_storage.insert(text_entity.clone(), Color([1.0, 1.0, 1.0, 1.0]));

    container_node.add(Node::new(Some(text_entity.clone()), None));

    container_node
}