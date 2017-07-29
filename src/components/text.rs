use std::sync::Arc;
use specs::{Component, VecStorage};
use rusttype::{Font, PositionedGlyph, Scale, point, Point};
use gfx;

pub struct Text<R: gfx::Resources> {
    pub font: Arc<Font<'static>>,
    pub scale: Scale,
    pub offset: Point<f32>,
    pub texture: Option<gfx::handle::ShaderResourceView<R, [f32; 4]>>,
    pub width: usize,
}

impl<R> Text<R>
    where R: gfx::Resources {
    pub fn new(font: Arc<Font<'static>>, size: f32) -> Text<R> {
        let scale = Scale { x: size, y: size };
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        Text{
            font: font,
            scale: scale,
            offset: offset,
            texture: None,
            width: 0,
        }
    }

    fn calc_text_width(&self, glyphs: &[PositionedGlyph]) -> f32 {
        glyphs.last().unwrap().pixel_bounding_box().unwrap().max.x as f32
    }

    pub fn set_text(&mut self, text: &str) {
        let font = self.font.clone();
        let glyphs = font.layout(text, self.scale, self.offset).collect::<Vec<PositionedGlyph<'static>>>();

        let pixel_height = self.scale.y.ceil() as usize;
        let width = self.calc_text_width(&glyphs) as usize;
        let mut pixel_data = vec![0_u8; 4 * width * pixel_height];
        let mapping_scale = 255.0;
        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                // v is the amount of the pixel covered
                // by the glyph, in the range 0.0 to 1.0
                g.draw(|x, y, v| {
                    let v = (v * mapping_scale + 0.5) as u8;
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    // There's still a possibility that the glyph clips the boundaries of the bitmap
                    if v > 0 && x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
                        let i = (x as usize + y as usize * width) * 4;
                        pixel_data[i] = 255;
                        pixel_data[i + 1] = 255;
                        pixel_data[i + 2] = 255;
                        pixel_data[i + 3] = v;
                    }
                })
            }
        }

        self.width = width;
    }
}

impl<R> Component for Text<R>
    where R: gfx::Resources {
    type Storage = VecStorage<Text<R>>;
}