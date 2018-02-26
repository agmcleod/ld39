use specs::{Entities, WriteStorage};
use components::{Color, Text, Transform};

pub struct TextStorage<'a> {
    pub entities: Entities<'a>,
    pub color_storage: WriteStorage<'a, Color>,
    pub text_storage: WriteStorage<'a, Text>,
    pub transform_storage: WriteStorage<'a, Transform>,
}
