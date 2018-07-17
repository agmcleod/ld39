use components::{GatheringRate, ResourceType};

pub fn get_total_gathering_rate(gathering_rate: &GatheringRate) -> i32 {
    gathering_rate.coal / ResourceType::Coal.get_efficiency_rate()
        + gathering_rate.oil / ResourceType::Oil.get_efficiency_rate()
        + gathering_rate.solar / ResourceType::Solar.get_efficiency_rate()
        + gathering_rate.hydro / ResourceType::Hydro.get_efficiency_rate()
}
