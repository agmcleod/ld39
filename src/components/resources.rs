use std::cmp;
use components::GathererType;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceType {
    Coal,
    Oil,
    Solar,
    Hydro,
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

    // TODO: Consider overflow
    pub fn withdraw_amount_for_type(&mut self, resource_type: ResourceType, amount: i32) -> i32 {
        match resource_type {
            ResourceType::Coal => {
                let amt = cmp::min(self.coal, amount);
                self.coal -= amt;
                amt
            }
            ResourceType::Oil => {
                let amt = cmp::min(self.oil, amount);
                self.oil -= amt;
                amt
            }
            ResourceType::Solar => {
                let amt = cmp::min(self.solar, amount);
                self.solar -= amt;
                amt
            }
            ResourceType::Hydro => {
                let amt = cmp::min(self.hydro, amount);
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
        self.coal = 50;
        self.oil = 0;
        self.solar = 0;
    }
}
