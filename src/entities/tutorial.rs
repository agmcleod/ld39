use cgmath::Vector2;
use components::{ui::TutorialUI, Actions, Color, EntityLookup, Node, Pulse, Rect, Shape, Text,
                 Transform, TutorialStep};
use entities::create_tooltip;
use specs::{Entities, Entity, Join, Read, ReadStorage, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use systems::logic;

pub fn create_step(
    entities: &Entities,
    color_storage: &mut WriteStorage<Color>,
    entity_lookup_storage: &Read<EntityLookup>,
    node_storage: &mut WriteStorage<Node>,
    pulse_storage: &mut WriteStorage<Pulse>,
    rect_storage: &mut WriteStorage<Rect>,
    shape_storage: &mut WriteStorage<Shape>,
    text_storage: &mut WriteStorage<Text>,
    tutorial_ui_storage: &mut WriteStorage<TutorialUI>,
    transform_storage: &mut WriteStorage<Transform>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    message: &str,
) {
    let pulse_shape = entities.create();

    let points = vec![
        Vector2 {
            x: x - 2.0,
            y: y - 2.0,
        },
        Vector2 {
            x: x - 2.0,
            y: y + h + 2.0,
        },
        Vector2 {
            x: x + w + 2.0,
            y: y + h + 2.0,
        },
        Vector2 {
            x: x + w + 2.0,
            y: y - 2.0,
        },
    ];

    let shape = Shape::new(points, [1.0, 1.0, 0.0, 0.0], false);
    shape_storage.insert(pulse_shape, shape).unwrap();
    let mut transform = Transform::visible_identity();
    transform.set_pos(0.0, 0.0, 4.0);
    transform_storage.insert(pulse_shape, transform).unwrap();
    tutorial_ui_storage
        .insert(pulse_shape, TutorialUI {})
        .unwrap();
    pulse_storage.insert(pulse_shape, Pulse::new(2.0)).unwrap();

    let tooltip = create_tooltip::create(
        &entities,
        color_storage,
        node_storage,
        rect_storage,
        text_storage,
        transform_storage,
        200.0,
        150.0,
        960,
        640,
        380,
        220,
        message.to_string(),
        Some(Color([0.0, 0.0, 0.0, 0.8])),
    );

    tutorial_ui_storage.insert(tooltip, TutorialUI {}).unwrap();

    let lookup = entity_lookup_storage.deref();
    let node = logic::get_root(&lookup, node_storage);

    node.add(pulse_shape);
    node.add(tooltip);
}

pub fn next_step(
    entities: &Entities,
    actions_storage: &mut Write<Actions>,
    tutorial_step_storage: &mut Write<TutorialStep>,
    tutorial_ui_storage: &ReadStorage<TutorialUI>,
    current_step: TutorialStep,
    next_step: TutorialStep,
) -> bool {
    let tutorial_step = tutorial_step_storage.deref_mut();
    if tutorial_step.as_string() == current_step.as_string() {
        for (entity, _) in (&**entities, tutorial_ui_storage).join() {
            entities.delete(entity).unwrap();
        }
        let actions = actions_storage.deref_mut();
        actions.dispatch(next_step.as_string());
        *tutorial_step = next_step;
        true
    } else {
        false
    }
}

pub fn clear_ui(
    entities: &Entities,
    tutorial_step_storage: &Write<TutorialStep>,
    tutorial_ui_storage: &ReadStorage<TutorialUI>,
    current_step: TutorialStep,
) {
    let tutorial_step = tutorial_step_storage.deref();
    if tutorial_step.as_string() == current_step.as_string() {
        for (entity, _) in (&**entities, tutorial_ui_storage).join() {
            entities.delete(entity).unwrap();
        }
    }
}
