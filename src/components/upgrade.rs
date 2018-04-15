use specs::{Component, VecStorage};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Locked,
    Researchable,
    Learning,
    Researched,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Upgrade {
    pub buff: Buff,
    pub time_to_research: f32,
    #[serde(default)]
    pub current_research_progress: f32,
    pub cost: usize,
    pub status: Status,
}

impl Upgrade {
    pub fn new(buff: Buff, time_to_research: f32, cost: usize, status: Status) -> Upgrade {
        Upgrade {
            buff,
            time_to_research,
            current_research_progress: 0.0,
            cost,
            status,
        }
    }

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
