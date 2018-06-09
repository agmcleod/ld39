use components::GathererType;

// This resource is used to cache the current amount gathering per tick, per resource
#[derive(Default)]
pub struct GatheringRate {
    pub coal: i32,
    pub oil: i32,
    pub solar: i32,
    pub hydro: i32,
}

impl GatheringRate {
    pub fn new() -> Self {
        GatheringRate {
            coal: 0,
            oil: 0,
            solar: 0,
            hydro: 0,
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

    pub fn reset(&mut self) {
        self.coal = 0;
        self.oil = 0;
        self.solar = 0;
        self.hydro = 0;
    }
}
