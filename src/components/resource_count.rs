use components::ResourceType;
use specs::{Component, VecStorage};

pub struct ResourceCount {
    pub resource_type: ResourceType,
}

impl Component for ResourceCount {
    type Storage = VecStorage<ResourceCount>;
}
