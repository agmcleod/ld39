use components::{Color, Text, Transform};
use gfx_glyph::HorizontalAlign;
use specs::Entity;
use storage_types::TextStorage;

pub fn create(
    storages: &mut TextStorage,
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

    let entity = storages.entities.create();
    storages
        .transform_storage
        .insert(
            entity.clone(),
            Transform::visible(x, y, z, w, h, 0.0, 1.0, 1.0),
        )
        .unwrap();
    storages.text_storage.insert(entity.clone(), text).unwrap();
    storages.color_storage.insert(entity, color).unwrap();

    entity
}
