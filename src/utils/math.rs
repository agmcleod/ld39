use std::time;

pub fn get_seconds(duration: &time::Duration) -> f32 {
    duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0
}

pub fn get_milliseconds(dur: &time::Duration) -> f32 {
    (dur.as_secs() * 1000) as f32 + dur.subsec_nanos() as f32 / 1000_000.0
}
