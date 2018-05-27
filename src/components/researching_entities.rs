use specs::Entity;

#[derive(Default)]
pub struct ResearchingEntities {
    pub entities: Vec<Entity>,
}

impl ResearchingEntities {
    pub fn new() -> Self {
        ResearchingEntities {
            entities: Vec::new(),
        }
    }
}
