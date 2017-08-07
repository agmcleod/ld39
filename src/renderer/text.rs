use std::collections::HashMap;
use gfx;
use gfx_device_gl;
use rusttype::Font;
use components;

use renderer::ColorFormat;

#[derive(Debug)]
pub struct GlyphCacheEntry<R: gfx::Resources>{
    pub view: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pub width: u16,
    pub height: u16,
}

pub fn create_texture_from_glyph<R, F>(glyph_cache: &mut HashMap<String, GlyphCacheEntry<R>>, font: &Font, text: &components::Text, factory: &mut F)
    where R: gfx::Resources, F: gfx::Factory<R> {
    let glyphs: Vec<_> = font.layout(text.text.as_ref(), text.scale, text.offset).collect();
    let pixel_height = text.scale.y.ceil() as usize;
    let width = text.calc_text_width(&glyphs) as usize;
    let mut pixel_data = vec![0u8; 4 * width * pixel_height];
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

    let kind = gfx::texture::Kind::D2(
        width as gfx::texture::Size,
        pixel_height as gfx::texture::Size,
        gfx::texture::AaMode::Single,
    );
    let tex = factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&pixel_data]);
    let (_, view) = tex.unwrap();
    glyph_cache.insert(text.text.clone(), GlyphCacheEntry{ view: view, width: width as u16, height: pixel_height as u16 });
}