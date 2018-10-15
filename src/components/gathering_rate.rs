use components::GathererType;

// This resource is used to cache the current amount gathering per tick, per resource
#[derive(Default)]
pub struct GatheringRate {
    pub coal: i32,
    pub oil: i32,
    pub solar: i32,
    pub hydro: i32,
    old_coal: i32,
    old_oil: i32,
    old_solar: i32,
    old_hydro: i32,
}

impl GatheringRate {
    pub fn new() -> Self {
        GatheringRate {
            coal: 0,
            oil: 0,
            solar: 0,
            hydro: 0,
            old_coal: 0,
            old_oil: 0,
            old_solar: 0,
            old_hydro: 0,
        }
    }

    pub fn add_to_resource_amount(&mut self, resource_type: &GathererType, amount: i32) {
        match *resource_type {
            GathererType::Coal => self.coal += amount,
            GathererType::Oil => self.oil += amount,
            GathererType::Solar => self.solar += amount,
            GathererType::Hydro => self.hydro += amount,
        }
    }

    pub fn changed(&self) -> bool {
        self.old_coal != self.coal || self.old_oil != self.oil || self.old_solar != self.solar
            || self.old_hydro != self.hydro
    }

    pub fn reset(&mut self) {
        if self.old_coal != self.coal {
            self.old_coal = self.coal;
        }
        if self.old_oil != self.oil {
            self.old_oil = self.oil;
        }
        if self.old_solar != self.solar {
            self.old_solar = self.solar;
        }
        if self.old_hydro != self.hydro {
            self.old_hydro = self.hydro;
        }
        self.coal = 0;
        self.oil = 0;
        self.solar = 0;
        self.hydro = 0;
    }
}
