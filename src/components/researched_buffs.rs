use entities::tech_tree::Buff;
use std::collections::HashSet;

#[derive(Default)]
pub struct ResearchedBuffs(pub HashSet<Buff>);
