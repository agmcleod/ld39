use std::time::Instant;
use specs::{Component, VecStorage};
use components::ResourceType;

#[derive(Debug)]
pub enum GathererType {
    Coal,
    Oil,
    Clean,
}

impl GathererType {
    pub fn get_type_for_resources_type(resource_type: &ResourceType) -> GathererType {
        match *resource_type {
            ResourceType::Coal => GathererType::Coal,
            ResourceType::Oil => GathererType::Oil,
            ResourceType::Clean => GathererType::Clean,
        }
    }

    pub fn get_build_cost(&self) -> usize {
        match *self {
            GathererType::Coal => 10,
            GathererType::Oil => 15,
            GathererType::Clean => 25,
        }
    }

    pub fn get_frames(&self) -> Vec<String> {
        match *self {
            GathererType::Coal => vec!["wheelbarrel.png".to_string()],
            GathererType::Oil => vec!["refinery_1.png".to_string(), "refinery_2.png".to_string(), "refinery_3.png".to_string(), "refinery_4.png".to_string(), "refinery_5.png".to_string(), "refinery_6.png".to_string(), "refinery_7.png".to_string(), "refinery_8.png".to_string()],
            GathererType::Clean => vec!["plant.png".to_string()],
        }
    }
}

pub struct Gatherer {
    pub gatherer_type: GathererType,
    pub gather_tick: Instant,
}

impl Gatherer {
    pub fn new(resource_type: &ResourceType) -> Gatherer {
        Gatherer{
            gatherer_type: GathererType::get_type_for_resources_type(resource_type),
            gather_tick: Instant::now(),
        }
    }
}

impl Component for Gatherer {
    type Storage = VecStorage<Gatherer>;
}