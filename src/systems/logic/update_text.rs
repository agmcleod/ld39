use components::Text;
use specs::{Component, Join, ReadStorage, WriteStorage};

pub fn update_text<'a, C>(
    message: String,
    text_storage: &mut WriteStorage<'a, Text>,
    identity_storage: &ReadStorage<'a, C>,
) where
    C: Component,
{
    for (text, _) in (text_storage, identity_storage).join() {
        text.set_text(message.clone());
    }
}

pub fn update_text_mut<'a, C>(
    message: String,
    text_storage: &mut WriteStorage<'a, Text>,
    identity_storage: &mut WriteStorage<'a, C>,
) where
    C: Component,
{
    for (text, _) in (text_storage, identity_storage).join() {
        text.set_text(message.clone());
    }
}
