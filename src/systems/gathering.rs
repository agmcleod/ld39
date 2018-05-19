use std::time::Instant;
use std::ops::{Deref, DerefMut};
use specs::{Join, Read, System, Write, WriteStorage};
use components::{Gatherer, GathererType, ResearchedBuffs, Resources, upgrade::Buff};
use utils::math;

pub struct Gathering;

impl Gathering {
    fn get_resource_gain(&self, gatherer_type: &GathererType) -> i32 {
        match *gatherer_type {
            GathererType::Coal => 4,
            GathererType::Oil => 4,
            GathererType::Solar => 6,
            GathererType::Hydro => 7,
        }
    }
}

impl<'a> System<'a> for Gathering {
    type SystemData = (
        WriteStorage<'a, Gatherer>,
        Read<'a, ResearchedBuffs>,
        Write<'a, Resources>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gatherer_storage, researched_buffs_storage, mut resources_storage) = data;

        let resources: &mut Resources = resources_storage.deref_mut();
        let researched_buffs = researched_buffs_storage.deref();

        for gatherer in (&mut gatherer_storage).join() {
            if math::get_seconds(&gatherer.gather_tick.elapsed()) >= 1.2 {
                gatherer.gather_tick = Instant::now();
                let mut amount = self.get_resource_gain(&gatherer.gatherer_type);
                if gatherer.has_adjancent_of_same_type
                    && researched_buffs.0.contains(&Buff::ResourceTrading)
                {
                    amount += 1;
                }
                resources.increase_resource_for_gatherer_type(&gatherer.gatherer_type, amount);
            }
        }
    }
}
