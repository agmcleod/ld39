use specs::{Component, HashMapStorage};
use std::collections::HashSet;
use self::super::Buff;

pub struct ResearchedBuffs(pub HashSet<Buff>);

impl Component for ResearchedBuffs {
    type Storage = HashMapStorage<Self>;
}
