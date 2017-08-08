use specs::{Component, Join, ReadStorage, WriteStorage};
use components::Text;

pub fn update_text<'a, C>(message: String, text_storage: &mut WriteStorage<'a, Text>, identity_storage: &ReadStorage<'a, C>)
    where C: Component {
    for (text, _) in (text_storage, identity_storage).join() {
        if text.text != message {
            text.set_text(message.clone());
        }
    }
}
