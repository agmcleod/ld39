use std::time::Instant;
use specs::{WriteStorage, Join, System};
use components::{AnimationSheet};
use utils::math;

pub struct AnimationSystem;

impl AnimationSystem {
    pub fn new() -> AnimationSystem {
        AnimationSystem{}
    }
}

impl<'a> System<'a> for AnimationSystem {
    type SystemData = (
        WriteStorage<'a, AnimationSheet>
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut animation_sheet_storage = data;

        for animation_sheet in (&mut animation_sheet_storage).join() {
            if math::get_mills(&animation_sheet.time_passed.elapsed()) >= animation_sheet.frame_time {
                animation_sheet.current_index += 1;
                animation_sheet.time_passed = Instant::now();
                if animation_sheet.current_index >= animation_sheet.get_current_animation().len() {
                    animation_sheet.current_index = 0;
                }
            }
        }
    }
}