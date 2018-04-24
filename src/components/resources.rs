use components::GathererType;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResourceType {
    Coal,
    Oil,
    Solar,
    Hydro,
}

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
    pub fn withdraw_all_for_type(&mut self, resource_type: ResourceType) -> i32 {
        match resource_type {
            ResourceType::Coal => {
                let amount = self.coal;
                self.coal = 0;
                amount
            }
            ResourceType::Oil => {
                let amount = self.oil;
                self.oil = 0;
                amount
            }
            ResourceType::Solar => {
                let amount = self.solar;
                self.solar = 0;
                amount
            }
            ResourceType::Hydro => {
                let amount = self.hydro;
                self.hydro = 0;
                amount
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

    pub fn increase_resource_for_gatherer_type(&mut self, gatherer_type: &GathererType) {
        match *gatherer_type {
            GathererType::Coal => self.coal += 1,
            GathererType::Oil => self.oil += 2,
            GathererType::Solar => self.solar += 4,
            GathererType::Hydro => self.hydro += 3,
        }
    }

    pub fn reset(&mut self) {
        self.coal = 50;
        self.oil = 0;
        self.solar = 0;
    }
}
