use specs::{Component, VecStorage};
use components::ResourceType;

pub enum GathererType {
    Coal,
    Oil,
    Clean,
}

impl GathererType {
    fn get_type_for_resources_type(resource_type: &ResourceType) -> GathererType {
        match *resource_type {
            ResourceType::Coal => GathererType::Coal,
            ResourceType::Oil => GathererType::Oil,
            ResourceType::Clean => GathererType::Clean,
        }
    }

    pub fn get_build_cost(&self) -> usize {
        match *self {
            GathererType::Coal => 15,
            GathererType::Oil => 25,
            GathererType::Clean => 50,
        }
    }

    pub fn get_frames(&self) -> Vec<String> {
        match *self {
            GathererType::Coal => vec!["wheelbarrel.png".to_string()],
            GathererType::Oil => vec!["refinery.png".to_string()],
            GathererType::Clean => vec!["plant.png".to_string()],
        }
    }
}

pub struct Gatherer {
    pub gatherer_type: GathererType,
}

impl Gatherer {
    pub fn new(resource_type: &ResourceType) -> Gatherer {
        Gatherer{
            gatherer_type: GathererType::get_type_for_resources_type(resource_type),
        }
    }
}

impl Component for Gatherer {
    type Storage = VecStorage<Gatherer>;
}