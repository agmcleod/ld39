use std::time::Instant;
use std::ops::{Deref, DerefMut};
use specs::{Join, Read, System, Write, WriteStorage};
use components::{Gatherer, GathererType, GatheringRate, ResearchedBuffs, Resources, upgrade::Buff};
use utils::math;

pub struct Gathering {
    gathering_tick: Instant,
}

impl Gathering {
    pub fn new () -> Self {
        Gathering{
            gathering_tick: Instant::now()
        }
    }

    fn get_resource_gain(&self, gatherer_type: &GathererType) -> i32 {
        match *gatherer_type {
            GathererType::Coal => 12,
            GathererType::Oil => 16,
            GathererType::Solar => 19,
            GathererType::Hydro => 22,
        }
    }
}

impl<'a> System<'a> for Gathering {
    type SystemData = (
        WriteStorage<'a, Gatherer>,
        Write<'a, GatheringRate>,
        Read<'a, ResearchedBuffs>,
        Write<'a, Resources>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut gatherer_storage,
            mut gathering_rate_storage,
            researched_buffs_storage,
            mut resources_storage,
        ) = data;

        let resources: &mut Resources = resources_storage.deref_mut();
        let researched_buffs = researched_buffs_storage.deref();

        let gathering_rate = gathering_rate_storage.deref_mut();
        gathering_rate.reset();

        let mut time_passed = false;
        for gatherer in (&mut gatherer_storage).join() {
            if math::get_seconds(&self.gathering_tick.elapsed()) >= 5.0 {
                time_passed = true;
                let mut amount = self.get_resource_gain(&gatherer.gatherer_type);
                if gatherer.has_adjancent_of_same_type
                    && researched_buffs.0.contains(&Buff::ResourceTrading)
                {
                    amount += 1;
                }

                if gatherer.gatherer_type == GathererType::Coal {
                    if researched_buffs.0.contains(&Buff::ConveyerBelts) {
                        amount += 1;
                    }
                    if researched_buffs.0.contains(&Buff::RoboticLoaders) {
                        amount += 1;
                    }
                } else if gatherer.gatherer_type == GathererType::Oil {
                    if researched_buffs.0.contains(&Buff::AutomatedRefiners) {
                        amount += 1;
                    }
                    if researched_buffs.0.contains(&Buff::Purifier) {
                        amount += 1;
                    }
                } else if gatherer.gatherer_type == GathererType::Hydro {
                    if researched_buffs.0.contains(&Buff::ReinforcedTurbines) {
                        amount += 1;
                    }
                } else if gatherer.gatherer_type == GathererType::Solar {
                    if researched_buffs.0.contains(&Buff::ImprovePanelTech) {
                        amount += 1;
                    }
                }

                gathering_rate.add_to_resource_amount(&gatherer.gatherer_type, amount);
            }
        }

        if time_passed {
            self.gathering_tick = Instant::now();
        }

        resources.increase_resource_for_gatherer_type(&GathererType::Coal, gathering_rate.coal);
        resources.increase_resource_for_gatherer_type(&GathererType::Oil, gathering_rate.oil);
        resources.increase_resource_for_gatherer_type(&GathererType::Solar, gathering_rate.solar);
        resources.increase_resource_for_gatherer_type(&GathererType::Hydro, gathering_rate.hydro);
    }
}
