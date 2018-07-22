// Idea with this component is to dispatch arbitrary actions
// potential maybe to refactor other more specific types

// Keeping a simple vec of strings for now, but could expand with traits to add action objects

// All actions are cleared after systems are processed

#[derive(Default)]
pub struct Actions {
    pub actions: Vec<String>,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            actions: Vec::new(),
        }
    }

    pub fn action_fired(&self, name: &str) -> bool {
        self.actions.contains(&String::from(name))
    }

    pub fn dispatch(&mut self, name: String) {
        self.actions.push(name);
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }
}
