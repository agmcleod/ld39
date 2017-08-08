use specs::{Component, HashMapStorage};

pub struct WinCount{
    pub count: usize,
}

impl WinCount {
    pub fn get_message(&self) -> String {
        if self.count > 1 {
            format!("Build {} solar plants", self.count)
        } else if self.count == 1 {
            "Build 1 solar plant".to_string()
        } else {
            "Were saved!".to_string()
        }
    }
}

impl Component for WinCount {
    type Storage = HashMapStorage<WinCount>;
}