#[derive(PartialEq)]
pub enum InternalState {
    Game,
    TechTree,
    Pause,
}

impl Default for InternalState {
    fn default() -> Self {
        InternalState::Game
    }
}
