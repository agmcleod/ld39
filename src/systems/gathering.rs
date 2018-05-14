use std::time::Instant;
use std::ops::DerefMut;
use specs::{Write, Join, System, WriteStorage};
use components::{Gatherer, Resources};
use utils::math;

pub struct Gathering;

impl<'a> System<'a> for Gathering {
    type SystemData = (WriteStorage<'a, Gatherer>, Write<'a, Resources>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut gatherer_storage, mut resources_storage) = data;

        let resources: &mut Resources = resources_storage.deref_mut();

        for gatherer in (&mut gatherer_storage).join() {
            if math::get_seconds(&gatherer.gather_tick.elapsed()) >= 1.2 {
                gatherer.gather_tick = Instant::now();
                resources.increase_resource_for_gatherer_type(&gatherer.gatherer_type);
            }
        }
    }
}
