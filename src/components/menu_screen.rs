use rand::{Rng, ThreadRng};
use specs::{Component, VecStorage};

const MIN_X: f32 = -280.0;
const MAX_X: f32 = 0.0;
const MIN_Y: f32 = -470.0;
const MAX_Y: f32 = 0.0;

pub struct MenuScreen {
    pub start_time_in_seconds: f32,
    pub animating: bool,
    pub start_pos: (f32, f32),
    pub end_pos: (f32, f32),
}

impl MenuScreen {
    pub fn new(start_time_in_seconds: f32, start_pos: (f32, f32), end_pos: (f32, f32)) -> Self {
        MenuScreen {
            start_time_in_seconds,
            animating: false,
            start_pos,
            end_pos,
        }
    }

    pub fn get_random_position(rng: &mut ThreadRng) -> (f32, f32) {
        (rng.gen_range(MIN_X, MAX_X), rng.gen_range(MIN_Y, MAX_Y))
    }
}

impl Component for MenuScreen {
    type Storage = VecStorage<Self>;
}
