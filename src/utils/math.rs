use std::time;

pub fn get_seconds(duration: &time::Duration) -> f32 {
    duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0
}