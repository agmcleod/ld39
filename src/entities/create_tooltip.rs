use components::{Color, Node, Rect, Text, Transform};
use specs::{Entities, Entity, WriteStorage};
use std::cmp;

pub fn create(
    entities: &Entities,
    color_storage: &mut WriteStorage<Color>,
    node_storage: &mut WriteStorage<Node>,
    rect_storage: &mut WriteStorage<Rect>,
    text_storage: &mut WriteStorage<Text>,
    transform_storage: &mut WriteStorage<Transform>,
    x: f32,
    y: f32,
    right_max: i32,
    bottom_max: i32,
    w: u16,
    h: u16,
    text: String,
    color: Option<Color>,
) -> Entity {
    let background = entities.create();
    let color = if let Some(color) = color {
        color
    } else {
        Color([0.0, 0.0, 0.0, 0.6])
    };

    transform_storage
        .insert(
            background.clone(),
            Transform::visible(0.0, 0.0, 0.0, w, h, 0.0, 1.0, 1.0),
        )
        .unwrap();
    color_storage.insert(background.clone(), color).unwrap();
    rect_storage.insert(background.clone(), Rect {}).unwrap();

    let tooltip_container = entities.create();
    let x = cmp::max(0, x as i32);
    let x = cmp::min(x, right_max - w as i32);

    let y = cmp::max(0, y as i32);
    let y = cmp::min(y, bottom_max - h as i32);

    transform_storage
        .insert(
            tooltip_container.clone(),
            Transform::visible(x as f32, y as f32, 50.0, w, h, 0.0, 1.0, 1.0),
        )
        .unwrap();
    let mut container_node = Node::new();
    container_node.add(background);

    let text_entity = entities.create();
    text_storage
        .insert(
            text_entity.clone(),
            Text::new_with_text(20.0, (w as f32 * 0.9) as u16, (h as f32 * 0.9) as u16, text),
        )
        .unwrap();
    transform_storage
        .insert(
            text_entity.clone(),
            Transform::visible(
                w as f32 * 0.05,
                h as f32 * 0.05,
                0.0,
                w - 5,
                h - 5,
                0.0,
                1.0,
                1.0,
            ),
        )
        .unwrap();
    color_storage
        .insert(text_entity.clone(), Color([1.0, 1.0, 1.0, 1.0]))
        .unwrap();

    container_node.add(text_entity);
    node_storage
        .insert(tooltip_container, container_node)
        .unwrap();

    tooltip_container
}
