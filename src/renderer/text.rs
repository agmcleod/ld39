use std::collections::HashMap;
use gfx;
use gfx_device_gl;
use rusttype::{Font, point, Scale};
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
    let v_metrics = font.v_metrics(text.scale);
    let mut caret = point(0.0, v_metrics.ascent);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

    let mut glyphs = Vec::new();
    let mut last_glyph_id = None;

    let mut pixel_height = text.scale.y.ceil();

    for c in text.text.chars() {
        if c.is_control() {
            match c {
                '\n' => {
                    caret = point(0.0, caret.y + advance_height);
                    pixel_height += advance_height;
                },
                _ => {}
            }
            continue;
        }

        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(text.scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(text.scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > text.size.x as i32 {
                caret = point(0.0, caret.y + advance_height);
                pixel_height += advance_height;
                glyph = glyph.into_unpositioned().positioned(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        glyphs.push(glyph);
    }

    let width = text.calc_text_width(&glyphs) as usize;
    let mut pixel_data = vec![0u8; 4 * width * pixel_height as usize];
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
    let tex = factory.create_texture_immutable_u8::<ColorFormat>(kind, gfx::texture::Mipmap::Allocated,&[&pixel_data]);
    let (_, view) = tex.unwrap();
    glyph_cache.insert(text.text.clone(), GlyphCacheEntry{ view: view, width: width as u16, height: pixel_height as u16 });
}