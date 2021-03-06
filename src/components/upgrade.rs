use specs::{Component, Entity, HashMapStorage, VecStorage};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Locked,
    Researchable,
    Learning,
    Researched,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Buff {
    Coal,
    Oil,
    Solar,
    ResourceTrading,
    ConveyerBelts,
    RoboticLoaders,
    PollutionFilters,
    FudgeTheNumbers,
    AutomatedRefiners,
    Purifier,
    Hydro,
    SalmonCannon,
    ReinforcedTurbines,
    PurchaseSolarCellCompany,
    ImprovePanelTech,
    SellPanelsToConsumers,
}

impl Buff {
    pub fn has_levels(&self) -> bool {
        match *self {
            Buff::ResourceTrading
            | Buff::ConveyerBelts
            | Buff::RoboticLoaders
            | Buff::AutomatedRefiners
            | Buff::Purifier
            | Buff::ReinforcedTurbines
            | Buff::ImprovePanelTech => true,
            _ => false,
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
