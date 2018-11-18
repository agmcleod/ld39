use std::collections::HashMap;

// Idea with this component is to dispatch arbitrary actions
// potential maybe to refactor other more specific types

// Keeping a simple HashMap of strings for now, but could expand with traits to add action objects

// All actions are cleared after systems are processed

#[derive(Default)]
pub struct Actions {
    pub actions: HashMap<String, String>,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            actions: HashMap::new(),
        }
    }

    pub fn action_fired(&self, name: &str) -> bool {
        self.actions.contains_key(&String::from(name))
    }

    pub fn dispatch(&mut self, name: String, value: String) {
        self.actions.insert(name, value);
    }

    pub fn get_payload(&self, name: &str) -> Option<&String> {
        self.actions.get(&String::from(name))
    }

    pub fn remove(&mut self, name: String) {
        self.actions.remove(&name);
    }
}
