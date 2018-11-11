use gfx_glyph::HorizontalAlign;
use specs::{Entities, Join, Read, System, WriteStorage};

use components::{Actions, DeltaTime, EntityLookup, Error, Color, Node, Text, Transform};
use entities::create_text;
use renderer::get_dimensions;
use storage_types::TextStorage;
use systems::logic;

pub struct Errors;

impl<'a> System<'a> for Errors {
    type SystemData = (
        Entities<'a>,
        Read<'a, Actions>,
        WriteStorage<'a, Color>,
        Read<'a, DeltaTime>,
        Read<'a, EntityLookup>,
        WriteStorage<'a, Error>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            actions_storage,
            mut color_storage,
            delta_time_storage,
            entity_lookup_storage,
            mut error_storage,
            mut node_storage,
            mut text_storage,
            mut transform_storage,
        ) = data;

        if actions_storage.action_fired("display_error") {
            let payload = actions_storage.get_payload("display_error").unwrap();
            let mut text_storage = TextStorage{
                entities: &entities,
                color_storage: &mut color_storage,
                text_storage: &mut text_storage,
                transform_storage: &mut transform_storage,
            };

            let dim = get_dimensions();
            let text_entity = create_text::create(
                &mut text_storage,
                payload.clone(),
                28.0,
                dim[0] / 2.0,
                200.0,
                9.0,
                dim[0] as u16,
                30,
                Color([1.0, 0.0, 0.0, 1.0]),
                Some(HorizontalAlign::Center),
            );
            error_storage.insert(text_entity, Error::new(4.0)).unwrap();
            let root = logic::get_root(&entity_lookup_storage, &mut node_storage);
            root.add(text_entity);
        }

        for (entity, error) in (&*entities, &mut error_storage).join() {
            error.tick -= delta_time_storage.dt;
            if error.tick <= 0.0 {
                entities.delete(entity).unwrap();
            }
        }
    }
}
