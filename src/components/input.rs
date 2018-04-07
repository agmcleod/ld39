use std::collections::HashMap;
use glutin::VirtualKeyCode;

#[derive(Debug)]
pub struct Input {
    pub hidpi_factor: f32,
    pub pressed_keys: HashMap<VirtualKeyCode, bool>,
    pub mouse_pos: (f32, f32),
    pub mouse_pressed: bool,
}

impl Input {
    pub fn new(hidpi_factor: f32, keys: Vec<VirtualKeyCode>) -> Input {
        let mut key_map: HashMap<VirtualKeyCode, bool> = HashMap::new();
        for key in keys {
            key_map.insert(key, false);
        }

        Input {
            hidpi_factor: hidpi_factor,
            pressed_keys: key_map,
            mouse_pos: (0.0, 0.0),
            mouse_pressed: false,
        }
    }
}
