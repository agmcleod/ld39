// Idea with this component is to dispatch arbitrary actions
// potential maybe to refactor other more specific types

// Keeping a simple vec of strings for now, but could expand with traits to add action objects

// All actions are cleared after systems are processed

#[derive(Default)]
pub struct Actions {
    actions: Vec<String>,
}

impl Actions {
    pub fn new() -> Self {
        Actions {
            actions: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, name: String) {
        self.actions.push(name);
    }

    pub fn next(&self) -> Option<String> {
        if self.actions.len() > 0 {
            let index = self.actions.len() - 1;
            Some((self.actions.get(index).unwrap()).clone())
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }
}
