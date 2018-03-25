use specs::{Component, VecStorage};
use rusttype::Scale;
use cgmath::{Vector2};

pub struct Text {
    pub scale: Scale,
    pub new_data: bool,
    pub text: String,
    pub visible: bool,
    pub size: Vector2<u16>,
}

impl Text {
    pub fn new(size: f32, w: u16, h: u16) -> Text {
        let scale = Scale { x: size, y: size };

        Text{
            scale: scale,
            new_data: false,
            text: "".to_string(),
            visible: true,
            size: Vector2{ x: w, y: h },
        }
    }

    pub fn new_with_text(size: f32, w: u16, h: u16, text: String) -> Text {
        let mut text_component = Text::new(size, w, h);
        text_component.text = text;
        text_component
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

impl Component for Text {
    type Storage = VecStorage<Text>;
}