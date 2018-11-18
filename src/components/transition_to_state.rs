use specs::{Component, HashMapStorage};

pub struct TransitionToState {
    pub state: String,
}

impl TransitionToState {
    pub fn new(state: String) -> Self {
        TransitionToState { state }
    }
}

impl Component for TransitionToState {
    type Storage = HashMapStorage<Self>;
}
