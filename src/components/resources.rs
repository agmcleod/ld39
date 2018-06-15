use std::cmp;
use components::GathererType;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceType {
    Coal,
    Oil,
    Solar,
    Hydro,
}

impl ResourceType {
    pub fn get_efficiency_rate(&self) -> i32 {
        match *self {
            ResourceType::Coal => 4,
            ResourceType::Oil => 3,
            ResourceType::Solar => 2,
            ResourceType::Hydro => 2,
        }
    }
}

#[derive(Default)]
pub struct Resources {
    pub coal: i32,
    pub oil: i32,
    pub solar: i32,
    pub hydro: i32,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            coal: 0,
            oil: 0,
            solar: 0,
            hydro: 0,
        }
    }

    pub fn withdraw_amount_for_type(&mut self, resource_type: ResourceType, amount: i32) -> i32 {
        match resource_type {
            ResourceType::Coal => {
                let mut amt = cmp::min(self.coal, amount);
                amt = amt / 4 * 4;
                self.coal -= amt;
                amt / 4
            }
            ResourceType::Oil => {
                let mut amt = cmp::min(self.oil, amount);
                amt = amt / 3 * 3;
                self.oil -= amt;
                amt / 3
            }
            ResourceType::Solar => {
                let mut amt = cmp::min(self.solar, amount);
                amt = amt / 2 * 2;
                self.solar -= amt;
                amt
            }
            ResourceType::Hydro => {
                let mut amt = cmp::min(self.hydro, amount);
                amt = amt / 2 * 2;
                self.hydro -= amt;
                amt
            }
        }
    }

    pub fn get_amount_for_type(&self, resource_type: &ResourceType) -> i32 {
        match *resource_type {
            ResourceType::Coal => self.coal,
            ResourceType::Oil => self.oil,
            ResourceType::Solar => self.solar,
            ResourceType::Hydro => self.hydro,
        }
    }

    pub fn increase_resource_for_gatherer_type(
        &mut self,
        gatherer_type: &GathererType,
        amount: i32,
    ) {
        match *gatherer_type {
            GathererType::Coal => self.coal += amount,
            GathererType::Oil => self.oil += amount,
            GathererType::Solar => self.solar += amount,
            GathererType::Hydro => self.hydro += amount,
        }
    }

    pub fn reset(&mut self) {
        self.coal = 0;
        self.oil = 0;
        self.solar = 0;
    }
}
