use specs::{Component, Entity, HashMapStorage, VecStorage};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Locked,
    Researchable,
    Learning,
    Researched,
}

#[derive(Copy, Clone, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum Buff {
    Coal,
    Oil,
    Solar,
    ResourceTrading(u32),
    ConveyerBelts(u32),
    RoboticLoaders(u32),
    PollutionFilters,
    FudgeTheNumbers,
    AutomatedRefiners(u32),
    Purifier(u32),
    Hydro,
    SalmonCannon,
    ReinforcedTurbines(u32),
    PurchaseSolarCellCompany,
    ImprovePanelTech(u32),
    SellPanelsToConsumers,
}

impl Buff {
    pub fn get_level(&self) -> Option<u32> {
        match *self {
            Buff::ResourceTrading(n) => Some(n),
            Buff::ConveyerBelts(n) => Some(n),
            Buff::RoboticLoaders(n) => Some(n),
            Buff::AutomatedRefiners(n) => Some(n),
            Buff::Purifier(n) => Some(n),
            Buff::ReinforcedTurbines(n) => Some(n),
            Buff::ImprovePanelTech(n) => Some(n),
            _ => None,
        }
    }

    pub fn has_levels(&self) -> bool {
        match *self {
            Buff::ResourceTrading(_) |
            Buff::ConveyerBelts(_) |
            Buff::RoboticLoaders(_) |
            Buff::AutomatedRefiners(_) |
            Buff::Purifier(_) |
            Buff::ReinforcedTurbines(_) |
            Buff::ImprovePanelTech(_) => true,
            _ => false,
        }
    }
}

impl PartialEq for Buff {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Buff::ResourceTrading(_) => {
                match *other {
                    Buff::ResourceTrading(_) => true,
                    _ => false,
                }
            },
            Buff::ConveyerBelts(_) => {
                match *other {
                    Buff::ConveyerBelts(_) => true,
                    _ => false,
                }
            },
            Buff::RoboticLoaders(_) => {
                match *other {
                    Buff::RoboticLoaders(_) => true,
                    _ => false,
                }
            },
            Buff::AutomatedRefiners(_) => {
                match *other {
                    Buff::AutomatedRefiners(_) => true,
                    _ => false,
                }
            },
            Buff::Purifier(_) => {
                match *other {
                    Buff::Purifier(_) => true,
                    _ => false,
                }
            },
            Buff::ReinforcedTurbines(_) => {
                match *other {
                    Buff::ReinforcedTurbines(_) => true,
                    _ => false,
                }
            },
            Buff::ImprovePanelTech(_) => {
                match *other {
                    Buff::ImprovePanelTech(_) => true,
                    _ => false,
                }
            },
            _ => *self == *other,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Upgrade {
    pub buff: Buff,
    pub time_to_research: f32,
    #[serde(default)]
    pub current_research_progress: f32,
    pub cost: i32,
    pub status: Status,
}

impl Upgrade {
    pub fn start_learning(&mut self) {
        self.status = Status::Learning;
    }
}

impl Component for Upgrade {
    type Storage = VecStorage<Upgrade>;
}

pub fn get_color_from_status(status: &Status) -> [f32; 4] {
    match *status {
        Status::Researchable => [1.0, 1.0, 1.0, 1.0],
        _ => [0.3, 0.3, 0.3, 1.0],
    }
}

pub struct LearnProgress {
    pub buff: Buff,
}

impl Component for LearnProgress {
    type Storage = HashMapStorage<Self>;
}

#[derive(Default)]
pub struct UpgradeLinesLookup {
    pub entities: HashMap<Entity, Vec<Entity>>,
}

impl UpgradeLinesLookup {
    pub fn new() -> Self {
        UpgradeLinesLookup {
            entities: HashMap::new(),
        }
    }
}
