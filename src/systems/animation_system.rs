use components::{AnimationSheet, DeltaTime};
use specs::{Join, Read, System, WriteStorage};
use std::ops::Deref;

pub struct AnimationSystem;

impl AnimationSystem {
    pub fn new() -> AnimationSystem {
        AnimationSystem {}
    }
}

impl<'a> System<'a> for AnimationSystem {
    type SystemData = (WriteStorage<'a, AnimationSheet>, Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut animation_sheet_storage, delta_time_storage) = data;
        let dt = delta_time_storage.deref().dt;

        for animation_sheet in (&mut animation_sheet_storage).join() {
            animation_sheet.time_passed += dt;
            if animation_sheet.time_passed >= animation_sheet.frame_length {
                animation_sheet.current_index += 1;
                animation_sheet.time_passed = 0.0;
                if animation_sheet.current_index >= animation_sheet.get_current_animation().len() {
                    animation_sheet.current_index = 0;
                }
            }
        }
    }
}
