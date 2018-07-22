#[derive(Debug, PartialEq)]
pub enum TutorialStep {
    SelectTile,
    BuildCoal,
    CoalGathered,
    SellResources,
    ResourcesSold,
    ContinueBuilding,
    ShowUpgrades,
    Upgrade,
    Resume,
    Objective,
}

impl TutorialStep {
    pub fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Default for TutorialStep {
    fn default() -> Self {
        TutorialStep::SelectTile
    }
}
