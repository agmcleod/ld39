use tech_tree::buff::Buff;

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

pub struct Coal {
    upgrade: Upgrade,
}

impl Coal {
    pub fn new() -> Coal {
        Coal{
            upgrade: Upgrade{
                time_to_research: 0.0,
                status: Status::Researched,
            }
        }
    }
}

impl Buff for Coal {

}