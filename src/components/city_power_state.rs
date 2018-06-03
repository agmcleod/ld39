use specs::Entity;

pub const CITY_POWER_STATE_COORDS: [(f32, f32); 4] =
    [(30.0, 32.0), (162.0, 32.0), (30.0, 50.0), (162.0, 50.0)];

#[derive(Default)]
pub struct CityPowerState {
    pub border_entities: Vec<Entity>,
    pub current_city_count: usize,
}

impl CityPowerState {
    pub fn new(border_entities: Vec<Entity>) -> Self {
        CityPowerState {
            border_entities,
            current_city_count: 1,
        }
    }
}
