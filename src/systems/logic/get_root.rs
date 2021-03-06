use components::{EntityLookup, Node};
use specs::WriteStorage;

pub fn get_root<'a, 'b>(
    lookup: &EntityLookup,
    nodes_storage: &'a mut WriteStorage<'b, Node>,
) -> &'a mut Node {
    let root = lookup.entities.get("root").unwrap();
    nodes_storage.get_mut(*root).unwrap()
}
