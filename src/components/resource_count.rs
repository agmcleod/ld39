use specs::{Component, VecStorage};
use components::ResourceType;

pub struct ResourceCount {
    pub resource_type: ResourceType,
}

impl Component for ResourceCount {
    type Storage = VecStorage<ResourceCount>;
}