use specs::Component;
use specs::HashMapStorage;
use tech_tree::upgrade::{Status, Upgrade};

pub struct Solar {
    upgrade: Upgrade,
}

impl Solar {
    pub fn new() -> Solar {
        Solar{
            upgrade: Upgrade{
                time_to_research: 90.0,
                status: Status::Locked,
            }
        }
    }
}

impl Component for Solar {
    type Storage = HashMapStorage<Solar>;
}
