use specs::{Entities, WriteStorage};

use components::{Color, EntityLookup, Fade, FadeMode, Node, Rect, Transform, TransitionToState};
use renderer::get_dimensions;
use systems::logic;

pub fn create(
    entities: &Entities,
    color_storage: &mut WriteStorage<Color>,
    fade_storage: &mut WriteStorage<Fade>,
    nodes_storage: &mut WriteStorage<Node>,
    rect_storage: &mut WriteStorage<Rect>,
    transform_storage: &mut WriteStorage<Transform>,
    transition_to_state_storage: Option<&mut WriteStorage<TransitionToState>>,
    state_to_transition_to: Option<String>,
    lookup: &EntityLookup,
    mode: FadeMode,
    duration: f32,
) {
    let entity = entities.create();
    let alpha = if mode == FadeMode::In {
        0.0
    } else {
        1.0
    };

    color_storage.insert(entity, Color([1.0, 1.0, 1.0, alpha])).unwrap();
    rect_storage.insert(entity, Rect{}).unwrap();
    fade_storage.insert(entity, Fade::new(mode, duration)).unwrap();

    let dim = get_dimensions();
    transform_storage.insert(entity, Transform::visible(
        0.0,
        0.0,
        50.0,
        dim[0] as u16,
        dim[1] as u16,
        0.0,
        1.0,
        1.0
    )).unwrap();

    if let Some(transition_to_state_storage) = transition_to_state_storage {
        transition_to_state_storage.insert(entity, TransitionToState::new(state_to_transition_to.unwrap())).unwrap();
    }

    logic::get_root(lookup, nodes_storage).add(entity);
}
