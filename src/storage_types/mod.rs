use specs::{Entities, WriteStorage};
use components::{Color, Text, Transform};

pub struct TextStorage<'a, 'b: 'a> {
    pub entities: &'a Entities<'a>,
    pub color_storage: &'a mut WriteStorage<'b, Color>,
    pub text_storage: &'a mut WriteStorage<'b, Text>,
    pub transform_storage: &'a mut WriteStorage<'b, Transform>,
}
