use specs::{Component, HashMapStorage};

#[derive(PartialEq)]
pub enum FadeMode {
    In,
    Out,
}

pub struct Fade {
    pub mode: FadeMode,
    pub start_tick: f32,
    pub tick: f32,
}

impl Fade {
    pub fn new(mode: FadeMode, tick: f32) -> Self {
        Fade{
            mode,
            start_tick: tick,
            tick,
        }
    }
}

impl Component for Fade {
    type Storage = HashMapStorage<Self>;
}
