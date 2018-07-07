use components::{Color, DeltaTime, FloatingText, Transform};
use specs::{Entities, Join, Read, System, WriteStorage};
use std::ops::Deref;

pub struct FloatingTextSystem;

impl FloatingTextSystem {
    pub fn new() -> Self {
        FloatingTextSystem {}
    }
}

impl<'a> System<'a> for FloatingTextSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Color>,
        Read<'a, DeltaTime>,
        WriteStorage<'a, FloatingText>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut color_storage,
            delta_time_storage,
            mut floating_text_storage,
            mut transform_storage,
        ) = data;

        let delta_time = delta_time_storage.deref();

        for (entity, color, floating_text, transform) in (
            &*entities,
            &mut color_storage,
            &mut floating_text_storage,
            &mut transform_storage,
        ).join()
        {
            floating_text.time_passed += delta_time.dt;
            let x = transform.get_pos().x;
            transform.set_pos2(x, 0.0 - (20.0 * floating_text.time_passed));

            color.0[3] = 1.0 - floating_text.time_passed;

            if floating_text.time_passed >= 1.0 {
                entities.delete(entity).unwrap();
            }
        }
    }
}
