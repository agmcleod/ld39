use specs::Component;
use specs::HashMapStorage;
use tech_tree::upgrade::{Status, Upgrade};

pub struct Oil {
    pub upgrade: Upgrade,
}

impl Oil {
    pub fn new() -> Oil {
        Oil{
            upgrade: Upgrade{
                time_to_research: 60.0,
                cost: 10,
                status: Status::Researchable,
            }
        }
    }

    pub fn get_cost(&self) -> usize {
        self.upgrade.cost
    }
}

impl Component for Oil {
    type Storage = HashMapStorage<Oil>;
}
