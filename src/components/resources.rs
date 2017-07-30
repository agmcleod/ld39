use std::cmp;
use specs::{Component, HashMapStorage};
use components::GathererType;

#[derive(Copy, Clone, PartialEq)]
pub enum ResourceType {
    Coal,
    Oil,
    Clean,
}

impl ResourceType {
    fn get_current_amount(&self, resources: &Resources) -> usize {
        resources.coal + resources.oil + resources.clean
    }
}

pub struct Resources {
    pub coal: usize,
    pub oil: usize,
    pub clean: usize,
    pub current_type: ResourceType,
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
        let coal = self.coal;
        let oil = self.oil;
        let clean = self.clean;

        if amount > coal + oil + clean {
            return 0
        }

        let mut m_amount = amount;
        if coal > m_amount {
            self.coal -= m_amount;
            return amount
        } else {
            self.coal = 0;
            m_amount -= coal;
        }

        if oil > m_amount {
            self.oil -= m_amount;
            return amount
        } else {
            self.oil = 0;
            m_amount -= oil;
        }

        self.clean -= m_amount;
        return amount
    }

    pub fn get_amount(&mut self) -> usize {
        self.current_type.get_current_amount(self)
    }

    pub fn get_amount_for_type(&self, resource_type: &ResourceType) -> usize {
        match *resource_type {
            ResourceType::Coal => self.coal,
            ResourceType::Oil => self.oil,
            ResourceType::Clean => self.clean,
        }
    }

    pub fn get_current_type(&self) -> ResourceType {
        self.current_type
    }

    pub fn increase_type_for_gatherer_type(&mut self, gatherer_type: &GathererType) {
        match *gatherer_type {
            GathererType::Coal => {
                self.coal += 1;
            },
            GathererType::Oil => {
                self.oil += 2;
            },
            GathererType::Clean => {
                self.clean += 4;
            },
        }
    }
}

impl Component for Resources {
    type Storage = HashMapStorage<Resources>;
}