use rand::{thread_rng, Rng};
use specs::{Component, VecStorage};
use systems::MENU_ANIMATION_OPTIONS;

pub struct MenuScreen {
    pub start_time_in_seconds: f32,
    pub animating: bool,
    pub animation_index: usize,
}

impl MenuScreen {
    pub fn new(start_time_in_seconds: f32) -> Self {
        let mut rng = thread_rng();
        MenuScreen {
            start_time_in_seconds,
            animating: false,
            animation_index: rng.gen_range(0, MENU_ANIMATION_OPTIONS.len()),
        }
    }
}

impl Component for MenuScreen {
    type Storage = VecStorage<Self>;
}
