use specs::{Component, VecStorage};
use components::Input;

pub struct Button {
    pub name: String,
    pub frames: [String; 2],
    pub mouse_is_over: bool,
    pub pressed: bool,
    pub disabled: bool,
}

impl Button {
    pub fn new(name: String, frames: [String; 2]) -> Button {
        Button {
            name: name,
            frames: frames,
            mouse_is_over: false,
            pressed: false,
            disabled: false,
        }
    }

    pub fn clicked(&mut self, input: &Input) -> bool {
        if input.mouse_pressed && !self.pressed && self.mouse_is_over {
            self.pressed = true;
            return false;
        } else if !input.mouse_pressed && self.pressed {
            self.pressed = false;
            // return on release
            if self.mouse_is_over {
                return true;
            }

            return false;
        }

        false
    }

    pub fn get_hover_frame(&self) -> &String {
        self.frames.get(1).unwrap()
    }

    pub fn get_default_frame(&self) -> &String {
        self.frames.get(0).unwrap()
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }
}

impl Component for Button {
    type Storage = VecStorage<Button>;
}
