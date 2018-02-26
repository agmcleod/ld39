mod coal;
mod oil;
mod solar;

pub use self::coal::*;
pub use self::oil::*;
pub use self::solar::*;

pub enum Status {
    Locked,
    Researchable,
    Learning,
    Researched,
}

pub struct Upgrade {
    pub time_to_research: f32,
    pub cost: usize,
    pub status: Status
}

pub fn get_color_from_status(status: &Status) -> [f32; 4] {
    match *status {
        Status::Locked => [183.0 / 256.0, 189.0 / 256.0, 196.0 / 256.0, 1.0],
        Status::Researchable => [135.0 / 256.0, 177.0 / 256.0, 232.0 / 256.0, 1.0],
        Status::Learning => [105.0 / 256.0, 199.0 / 256.0, 113.0 / 256.0, 1.0],
        Status::Researched => [237.0 / 256.0, 154.0 / 256.0, 154.0 / 256.0, 1.0],
    }
}