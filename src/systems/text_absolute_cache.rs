use specs::{Entities, Join, System, WriteStorage, ReadStorage};
use std::sync::{Arc, Mutex};
use components::{Text, Transform};
use scene::Node;

pub struct TextAbsoluteCache {
    pub scene: Arc<Mutex<Node>>,
}

impl <'a>System<'a> for TextAbsoluteCache {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Text>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, text_strorage, mut transform_storage) = data;

        for (entity, text) in (&*entities, &text_strorage).join() {
            let absolute_pos = if transform_storage.get(entity).unwrap().dirty_pos {
                let scene = self.scene.lock().unwrap();
                Some(scene.get_absolute_pos(&entity, &transform_storage))
            } else {
                None
            };

            if let Some(absolute_pos) = absolute_pos {
                transform_storage.get_mut(entity).unwrap().set_absolute_pos(absolute_pos);
            }
        }
    }
}
