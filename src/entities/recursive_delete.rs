use components::Node;
use specs::{Entities, Entity, WriteStorage};

pub fn recursive_delete(entities: &Entities, node_storage: &WriteStorage<Node>, entity: &Entity) {
    if let Some(node) = node_storage.get(*entity) {
        for entity in &node.entities {
            recursive_delete(entities, node_storage, &entity);
        }
    }

    entities.delete(*entity).unwrap();
}
