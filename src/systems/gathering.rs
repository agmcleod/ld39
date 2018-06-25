use components::{upgrade::Buff, Color, FloatingText, Gatherer, GathererType, GatheringRate,
                 ResearchedBuffs, Resources, Text, Transform};
use entities::create_text;
use scene::Node;
use specs::{Entities, Join, Read, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use storage_types::TextStorage;
use utils::math;

pub struct Gathering {
    gathering_tick: Instant,
    scene: Arc<Mutex<Node>>,
}

impl Gathering {
    pub fn new(scene: Arc<Mutex<Node>>) -> Self {
        Gathering {
            gathering_tick: Instant::now(),
            scene,
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
        Entities<'a>,
        WriteStorage<'a, Color>,
        WriteStorage<'a, FloatingText>,
        WriteStorage<'a, Gatherer>,
        Write<'a, GatheringRate>,
        Read<'a, ResearchedBuffs>,
        Write<'a, Resources>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut color_storage,
            mut floating_text_storage,
            mut gatherer_storage,
            mut gathering_rate_storage,
            researched_buffs_storage,
            mut resources_storage,
            mut text_storage,
            mut transform_storage,
        ) = data;

        let resources: &mut Resources = resources_storage.deref_mut();
        let researched_buffs = researched_buffs_storage.deref();

        let gathering_rate = gathering_rate_storage.deref_mut();
        let mut scene = self.scene.lock().unwrap();

        if math::get_seconds(&self.gathering_tick.elapsed()) >= 5.0 {
            gathering_rate.reset();
            for (entity, gatherer) in (&*entities, &mut gatherer_storage).join() {
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

                let mut entity_node = scene.get_node_for_entity(entity).unwrap();
                let mut text_storage = TextStorage {
                    entities: &entities,
                    color_storage: &mut color_storage,
                    text_storage: &mut text_storage,
                    transform_storage: &mut transform_storage,
                };

                let floating_text = create_text::create(
                    &mut text_storage,
                    format!("{}", amount),
                    22.0,
                    0.0,
                    0.0,
                    0.0,
                    50,
                    18,
                    Color([0.0, 0.6, 0.0, 1.0]),
                );
                floating_text_storage
                    .insert(floating_text.clone(), FloatingText::new())
                    .unwrap();
                entity_node
                    .sub_nodes
                    .push(Node::new(Some(floating_text), None));
            }
            self.gathering_tick = Instant::now();

            resources.increase_resource_for_gatherer_type(&GathererType::Coal, gathering_rate.coal);
            resources.increase_resource_for_gatherer_type(&GathererType::Oil, gathering_rate.oil);
            resources
                .increase_resource_for_gatherer_type(&GathererType::Solar, gathering_rate.solar);
            resources
                .increase_resource_for_gatherer_type(&GathererType::Hydro, gathering_rate.hydro);
        }
    }
}
