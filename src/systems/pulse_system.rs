use components::{DeltaTime, Pulse, Shape};
use specs::{Join, Read, System, WriteStorage};
use std::ops::Deref;

pub struct PulseSystem;

impl<'a> System<'a> for PulseSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Pulse>,
        WriteStorage<'a, Shape>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta_time_storage, mut pulse_storage, mut shape_storage) = data;

        let dt = delta_time_storage.deref().dt;

        for (pulse, shape) in (&mut pulse_storage, &mut shape_storage).join() {
            pulse.time += dt;
            let half = pulse.rate / 2.0;
            let mut color = shape.color.clone();
            if pulse.time <= half {
                color[3] = pulse.time / half;
            } else {
                color[3] = 1.0 - (pulse.time - half) / half;
                if pulse.time >= pulse.rate {
                    pulse.time = 0.0;
                }
            }

            shape.set_color(color);
        }
    }
}
