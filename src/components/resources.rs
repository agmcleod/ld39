use std::cmp;
use specs::{Component, HashMapStorage};
use components::GathererType;

#[derive(Copy, Clone)]
pub enum ResourceType {
    Coal,
    Oil,
    Clean,
}

impl ResourceType {
    fn get_ratio(&self) -> f32 {
        match *self {
            ResourceType::Coal => 0.3,
            ResourceType::Oil => 0.6,
            ResourceType::Clean => 1.0,
        }
    }

    fn get_current_amount<'a>(&self, resources: &'a mut Resources) -> &'a mut usize {
        match *self {
            ResourceType::Coal => &mut resources.coal,
            ResourceType::Oil => &mut resources.oil,
            ResourceType::Clean => &mut resources.clean,
        }
    }
}

pub struct Resources {
    pub coal: usize,
    pub oil: usize,
    pub clean: usize,
    current_type: ResourceType,
}

impl Resources {
    pub fn new() -> Resources {
        Resources{
            coal: 50,
            oil: 0,
            clean: 0,
            current_type: ResourceType::Coal,
        }
    }

    pub fn get_resources(&mut self, amount: usize) -> usize {
        let current_type = self.current_type.clone();
        let current_amount = current_type.get_current_amount(self);

        if *current_amount == 0 {
            return 0
        }

        if amount > *current_amount {
            let cp = *current_amount as f32;
            *current_amount = 0;
            return cmp::max(
                (cp * current_type.get_ratio()).round() as usize
                , 1
            )
        }

        *current_amount -= amount;
        (amount as f32 * current_type.get_ratio()).round() as usize
    }

    pub fn get_resources_to_buy(&mut self, amount: usize) -> usize {
        let current_type = self.current_type.clone();
        let current_amount = current_type.get_current_amount(self);

        if *current_amount < amount {
            return 0
        }

        *current_amount -= amount;
        amount
    }

    pub fn get_current_type(&self) -> &ResourceType {
        &self.current_type
    }

    pub fn increase_type_for_gatherer_type(&mut self, gatherer_type: &GathererType) {
        match *gatherer_type {
            GathererType::Coal => {
                self.coal += 1;
            },
            GathererType::Oil => {
                self.oil += 1;
            },
            GathererType::Clean => {
                self.clean += 1;
            },
        }
    }
}

impl Component for Resources {
    type Storage = HashMapStorage<Resources>;
}