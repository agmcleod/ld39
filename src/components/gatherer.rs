use std::time::Instant;
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GathererType {
    Coal,
    Oil,
    Solar,
    Hydro,
}

impl GathererType {
    pub fn get_build_cost(&self) -> i32 {
        match *self {
            GathererType::Coal => 10,
            GathererType::Oil => 15,
            GathererType::Solar => 25,
            GathererType::Hydro => 20,
        }
    }

    pub fn get_frames(&self) -> Vec<String> {
        match *self {
            GathererType::Coal => vec!["wheelbarrel.png".to_string()],
            GathererType::Oil => vec![
                "refinery_1.png".to_string(),
                "refinery_2.png".to_string(),
                "refinery_3.png".to_string(),
                "refinery_4.png".to_string(),
                "refinery_5.png".to_string(),
                "refinery_6.png".to_string(),
                "refinery_7.png".to_string(),
                "refinery_8.png".to_string(),
            ],
            GathererType::Solar => vec!["plant.png".to_string()],
            GathererType::Hydro => vec!["hydro.png".to_string()],
        }
    }

    pub fn get_pollution_amount(&self) -> i32 {
        match *self {
            GathererType::Coal => 10,
            GathererType::Oil => 8,
            GathererType::Hydro => 5,
            _ => 0,
        }
    }
}

pub struct Gatherer {
    pub gatherer_type: GathererType,
    pub gather_tick: Instant,
    pub pollution: i32,
    pub has_adjancent_of_same_type: bool,
}

impl Gatherer {
    pub fn new(gatherer_type: GathererType, pollution: i32) -> Gatherer {
        Gatherer {
            gatherer_type: gatherer_type,
            gather_tick: Instant::now(),
            pollution,
            has_adjancent_of_same_type: false,
        }
    }
}

impl Component for Gatherer {
    type Storage = VecStorage<Gatherer>;
}
