use specs::{Component, VecStorage};
use rusttype::{PositionedGlyph, Scale};
use cgmath::{Matrix4, SquareMatrix, Transform, Vector2, Vector3};
use renderer::get_ortho;

pub struct Text {
    pub scale: Scale,
    pub new_data: bool,
    pub text: String,
    pub visible: bool,
    pub size: Vector2<u16>,
    pub draw_transform: Matrix4<f32>,
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
            draw_transform: Matrix4::identity(),
        }
    }

    pub fn new_with_absolute_position(size: f32, w: u16, h: u16, position: Vector3<f32>, text: String) -> Text {
        let mut text_component = Text::new(size, w, h);
        text_component.set_transform(position);
        text_component.text = text;
        text_component
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn set_transform(&mut self, absolute_position: Vector3<f32>) {
        let projection = get_ortho();
        let transform = Matrix4::from_translation(absolute_position);

        self.draw_transform = projection * transform;
    }
}

impl Component for Text {
    type Storage = VecStorage<Text>;
}