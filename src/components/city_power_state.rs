// used to keep track of the entities that we can add new power bar sprites to
#[derive(Default)]
pub struct CityPowerState {
    pub current_city_count: usize,
}

impl CityPowerState {
    pub fn new() -> Self {
        CityPowerState {
            current_city_count: 1,
        }
    }
}
