use std::collections::HashMap;
use specs::{Component, VecStorage};
use std::time::Instant;

pub struct AnimationSheet {
    pub animations: HashMap<String, Vec<String>>,
    pub current_animation: String,
    pub current_index: usize,
    pub frame_time: f32,
    pub time_passed: Instant,
}

impl AnimationSheet {
    pub fn new(frame_time: f32) -> AnimationSheet {
        AnimationSheet{
            animations: HashMap::new(),
            current_animation: String::new(),
            current_index: 0,
            frame_time: frame_time,
            time_passed: Instant::now(),
        }
    }

    pub fn add_animation(&mut self, name: String, frames: Vec<String>) {
        self.animations.insert(name, frames);
    }

    pub fn get_current_animation(&self) -> &Vec<String> {
        self.animations.get(&self.current_animation).unwrap()
    }

    pub fn get_current_frame(&self) -> &String {
        self.get_current_animation()
            .get(self.current_index).unwrap()
    }

    pub fn set_current_animation(&mut self, frame_name: String) {
        self.current_animation = frame_name;
        self.current_index = 0;
    }
}

impl Component for AnimationSheet {
    type Storage = VecStorage<AnimationSheet>;
}