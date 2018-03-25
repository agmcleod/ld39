use specs::{Component, VecStorage};

mod researched_buffs;

pub use self::researched_buffs::*;

#[derive(Debug, PartialEq)]
pub enum Status {
    Locked,
    Researchable,
    Learning,
    Researched,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Buff {
    Coal,
    Oil,
    Solar,
}

pub struct Upgrade {
    pub buff: Buff,
    pub time_to_research: f32,
    pub current_research_progress: f32,
    pub cost: usize,
    pub status: Status
}


impl Upgrade {
    pub fn new(buff: Buff, time_to_research: f32, cost: usize, status: Status) -> Upgrade {
        Upgrade{
            buff,
            time_to_research,
            current_research_progress: 0.0,
            cost,
            status
        }
    }

    pub fn start_learning(&mut self) {
        self.status = Status::Learning;
    }
}

impl Component for Upgrade {
    type Storage = VecStorage<Upgrade>;
}

pub fn get_color_from_status(status: &Status) -> [f32; 4] {
    match *status {
        Status::Locked => [183.0 / 256.0, 189.0 / 256.0, 196.0 / 256.0, 1.0],
        Status::Researchable => [135.0 / 256.0, 177.0 / 256.0, 232.0 / 256.0, 1.0],
        Status::Learning => [105.0 / 256.0, 199.0 / 256.0, 113.0 / 256.0, 1.0],
        Status::Researched => [237.0 / 256.0, 154.0 / 256.0, 154.0 / 256.0, 1.0],
    }
}
