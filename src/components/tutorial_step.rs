use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub enum TutorialStep {
    NotStarted,
    SelectTile,
    BuildCoal(f32, f32),
    CoalGathered,
    ResourcesSold,
    ShowUpgrades,
    Upgrade,
    Resume,
    Objective(f32),
}

impl Display for TutorialStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TutorialStep::BuildCoal(_, _) => write!(f, "BuildCoal"),
            TutorialStep::Objective(_) => write!(f, "Objective"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl TutorialStep {
    pub fn as_string(&self) -> String {
        format!("{}", self)
    }
}

impl Default for TutorialStep {
    fn default() -> Self {
        TutorialStep::NotStarted
    }
}
