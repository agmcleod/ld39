use components::{Color, Text, Transform};
use gfx_glyph::HorizontalAlign;
use specs::{Entity, Entities, LazyUpdate, Read};

pub fn create(
    entities: &Entities,
    lazy: &Read<LazyUpdate>,
    text: String,
    size: f32,
    x: f32,
    y: f32,
    z: f32,
    w: u16,
    h: u16,
    color: Color,
    align: Option<HorizontalAlign>,
) -> Entity {
    let mut text = Text::new_with_text(size, w, h, text);
    if let Some(align) = align {
        text = text.align(align);
    }

    lazy.create_entity(entities)
        .with(
            Transform::visible(x, y, z, w, h, 0.0, 1.0, 1.0)
        )
        .with(text)
        .with(color)
        .build()
}
