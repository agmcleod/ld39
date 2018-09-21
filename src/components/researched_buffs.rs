use entities::tech_tree::Buff;
use std::collections::HashMap;

#[derive(Default)]
pub struct ResearchedBuffs(pub HashMap<Buff, u32>);
