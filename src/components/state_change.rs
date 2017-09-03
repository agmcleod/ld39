use specs::{Component, HashMapStorage};

pub struct StateChange {
    pub state: String,
    pub action: String,
}

impl Clone for StateChange {
    fn clone(&self) -> StateChange {
        StateChange{
            state: self.state.clone(),
            action: self.action.clone(),
        }
    }
}

impl StateChange {
    pub fn new() -> StateChange {
        StateChange{
            state: "".to_string(),
            action: "".to_string(),
        }
    }

    pub fn reset(&mut self) {
        self.state = "".to_string();
        self.action = "".to_string();
    }
}

impl Component for StateChange {
    type Storage = HashMapStorage<StateChange>;
}