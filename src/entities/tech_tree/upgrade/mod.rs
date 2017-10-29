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
    time_to_research: f32,
    status: Status
}
