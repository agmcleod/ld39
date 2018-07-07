use components::{EntityLookup, Node, Text, Transform};
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};
use std::ops::Deref;

pub struct TextAbsoluteCache;

impl<'a> System<'a> for TextAbsoluteCache {
    type SystemData = (
        Entities<'a>,
        Read<'a, EntityLookup>,
        WriteStorage<'a, Node>,
        ReadStorage<'a, Text>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, entity_lookup_storage, node_storage, text_strorage, mut transform_storage) =
            data;

        let lookup = entity_lookup_storage.deref();
        let root_entity = lookup.get("root").unwrap();

        for (entity, _) in (&*entities, &text_strorage).join() {
            let absolute_pos = if transform_storage.get(entity).unwrap().dirty_pos {
                Some(Node::get_absolute_pos(
                    root_entity,
                    &entity,
                    &transform_storage,
                    &node_storage,
                ))
            } else {
                None
            };

            if let Some(absolute_pos) = absolute_pos {
                transform_storage
                    .get_mut(entity)
                    .unwrap()
                    .set_absolute_pos(absolute_pos);
            }
        }
    }
}
