use specs::{Component, VecStorage};
use rusttype::{Font, PositionedGlyph, Scale, point, Point};
use cgmath::Vector2;

pub struct Text {
    pub scale: Scale,
    pub offset: Point<f32>,
    pub new_data: bool,
    pub text: String,
    pub visible: bool,
    pub size: Vector2<u16>,
}

impl Text {
    pub fn new(font: &Font, size: f32) -> Text {
        let scale = Scale { x: size, y: size };
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        Text{
            scale: scale,
            offset: offset,
            new_data: false,
            text: "".to_string(),
            visible: true,
            size: Vector2{ x: size as u16, y: size as u16 },
        }
    }

    pub fn new_with_size(font: &Font, size: f32, w: u16, h: u16,) -> Text {
        let mut text_component = Text::new(font, size);
        text_component.size.x = w;
        text_component.size.y = w;
        text_component
    }

    pub fn new_with_text(font: &Font, size: f32, w: u16, h: u16, text: String) -> Text {
        let mut text_component = Text::new_with_size(font, size, w, h);
        text_component.set_text(text);
        text_component
    }

    pub fn calc_text_width(&self, glyphs: &[PositionedGlyph]) -> f32 {
        glyphs.last().unwrap().pixel_bounding_box().unwrap().max.x as f32
    }

    pub fn set_text(&mut self, text: String) {
        if self.text != text {
            self.new_data = true;
            self.text = text;
        }
    }
}

impl Component for Text {
    type Storage = VecStorage<Text>;
}