use components::{Color, PowerBar, Rect, Transform};
use specs::Entity;
use storage_types::PowerBarStorage;

pub fn create(power_bar_storage: &mut PowerBarStorage, x: f32, y: f32) -> Entity {
    let entity = power_bar_storage.entities.create();
    power_bar_storage
        .power_bar_storage
        .insert(entity, PowerBar::new())
        .unwrap();
    power_bar_storage
        .transform_storage
        .insert(
            entity,
            Transform::visible(
                x,
                y,
                1.0,
                PowerBar::get_max_width() as u16,
                24,
                0.0,
                1.0,
                1.0,
            ),
        )
        .unwrap();
    power_bar_storage
        .rect_storage
        .insert(entity, Rect::new())
        .unwrap();
    power_bar_storage
        .color_storage
        .insert(entity, Color([0.0, 1.0, 0.0, 1.0]))
        .unwrap();

    entity
}
