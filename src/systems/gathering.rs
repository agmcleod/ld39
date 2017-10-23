use std::time::Instant;
use std::ops::DerefMut;
use specs::{FetchMut, Join, WriteStorage, System};
use components::{Gatherer, Resources};
use utils::math;

pub struct Gathering;

impl<'a> System<'a> for Gathering {
    type SystemData = (
        WriteStorage<'a, Gatherer>,
        FetchMut<'a, Resources>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gatherer_storage, mut resources_storage) = data;

        let resources: &mut Resources = resources_storage.deref_mut();

        for gatherer in (&mut gatherer_storage).join() {
            if math::get_seconds(&gatherer.gather_tick.elapsed()) >= 1.2 {
                gatherer.gather_tick = Instant::now();
                resources.increase_type_for_gatherer_type(&gatherer.gatherer_type);
            }
        }
    }
}