#[derive(PartialEq)]
pub enum InternalState {
    Game,
    TechTree,
    Transition,
    Pause,
}

impl Default for InternalState {
    fn default() -> Self {
        InternalState::Game
    }
}
