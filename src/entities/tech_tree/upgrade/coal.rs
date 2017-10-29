use specs::Component;
use specs::HashMapStorage;
use tech_tree::upgrade::{Status, Upgrade};

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

impl Component for Coal {
    type Storage = HashMapStorage<Coal>;
}
