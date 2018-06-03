use specs::{Entities, WriteStorage};
use components::{Color, PowerBar, Rect, Text, Transform};

pub struct TextStorage<'a, 'b: 'a> {
    pub entities: &'a Entities<'a>,
    pub color_storage: &'a mut WriteStorage<'b, Color>,
    pub text_storage: &'a mut WriteStorage<'b, Text>,
    pub transform_storage: &'a mut WriteStorage<'b, Transform>,
}

pub struct PowerBarStorage<'a, 'b: 'a> {
    pub entities: &'a Entities<'a>,
    pub color_storage: &'a mut WriteStorage<'b, Color>,
    pub power_bar_storage: &'a mut WriteStorage<'b, PowerBar>,
    pub rect_storage: &'a mut WriteStorage<'b, Rect>,
    pub transform_storage: &'a mut WriteStorage<'b, Transform>,
}
