use gfx_glyph::HorizontalAlign;
use specs::{Entities, Join, Read, LazyUpdate, System, Write, WriteStorage};

use components::{Actions, Color, DeltaTime, EntityLookup, Error, Node, Text, Transform};
use entities::create_text;
use renderer::get_dimensions;
use systems::logic;

pub struct Errors;

impl<'a> System<'a> for Errors {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Write<'a, Actions>,
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
            lazy,
            mut actions_storage,
            mut color_storage,
            delta_time_storage,
            entity_lookup_storage,
            mut error_storage,
            mut node_storage,
            mut text_storage,
            mut transform_storage,
        ) = data;

        if actions_storage.action_fired("display_error") {
            let payload = actions_storage
                .get_payload("display_error")
                .unwrap()
                .clone();
            actions_storage.remove("display_error".to_string());

            let dim = get_dimensions();
            let text_entity = create_text::create(
                &entities,
                &lazy,
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
