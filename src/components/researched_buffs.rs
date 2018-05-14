use std::collections::HashSet;
use entities::tech_tree::Buff;

#[derive(Default)]
pub struct ResearchedBuffs(pub HashSet<Buff>);
