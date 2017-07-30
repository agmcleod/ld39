use specs::{Component, VecStorage};
use rusttype::{Font, PositionedGlyph, Scale, point, Point};

pub struct Text {
    pub scale: Scale,
    pub offset: Point<f32>,
    pub new_data: bool,
    pub text: String,
    pub visible: bool,
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
            visible: true
        }
    }

    pub fn new_from(scale: Scale, offset: Point<f32>) -> Text {
        Text{
            scale: scale,
            offset: offset,
            new_data: false,
            text: "".to_string(),
            visible: true,
        }
    }

    pub fn calc_text_width(&self, glyphs: &[PositionedGlyph]) -> f32 {
        glyphs.last().unwrap().pixel_bounding_box().unwrap().max.x as f32
    }

    pub fn set_text(&mut self, text: String) {
        self.new_data = true;
        self.text = text;
    }
}

impl Component for Text {
    type Storage = VecStorage<Text>;
}