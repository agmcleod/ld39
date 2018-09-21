use components::{ui::TutorialUI, upgrade::Buff, Actions, Color, DeltaTime, FloatingText, Gatherer,
                 GathererType, GatheringRate, Node, ResearchedBuffs, Resources, Text, Transform,
                 TutorialStep};
use entities::{create_text, tutorial};
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use storage_types::TextStorage;
use systems::TICK_RATE;

pub struct Gathering {
    gathering_tick: f32,
}

impl Gathering {
    pub fn new() -> Self {
        Gathering {
            gathering_tick: 0.0,
        }
    }

    fn get_resource_gain(&self, gatherer_type: &GathererType) -> i32 {
        match *gatherer_type {
            GathererType::Coal => 12,
            GathererType::Oil => 16,
            GathererType::Solar => 14,
            GathererType::Hydro => 22,
        }
    }
}

impl<'a> System<'a> for Gathering {
    type SystemData = (
        Entities<'a>,
        Write<'a, Actions>,
        WriteStorage<'a, Color>,
        Read<'a, DeltaTime>,
        WriteStorage<'a, FloatingText>,
        WriteStorage<'a, Gatherer>,
        Write<'a, GatheringRate>,
        WriteStorage<'a, Node>,
        Read<'a, ResearchedBuffs>,
        Write<'a, Resources>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
        Write<'a, TutorialStep>,
        ReadStorage<'a, TutorialUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut actions_storage,
            mut color_storage,
            delta_time_storage,
            mut floating_text_storage,
            mut gatherer_storage,
            mut gathering_rate_storage,
            mut node_storage,
            researched_buffs_storage,
            mut resources_storage,
            mut text_storage,
            mut transform_storage,
            mut tutorial_step_storage,
            tutorial_ui_storage,
        ) = data;

        let resources: &mut Resources = resources_storage.deref_mut();
        let researched_buffs = researched_buffs_storage.deref();

        let gathering_rate = gathering_rate_storage.deref_mut();

        let dt = delta_time_storage.deref().dt;
        self.gathering_tick += dt;

        if self.gathering_tick >= TICK_RATE {
            gathering_rate.reset();
            for (entity, gatherer) in (&*entities, &mut gatherer_storage).join() {
                let mut amount = self.get_resource_gain(&gatherer.gatherer_type);
                if gatherer.has_adjancent_of_same_type
                    && researched_buffs.0.contains_key(&Buff::ResourceTrading)
                {
                    let level = researched_buffs.0.get(&Buff::ResourceTrading).unwrap();
                    amount += *level as i32;
                }

                if gatherer.gatherer_type == GathererType::Coal {
                    if let Some(n) = researched_buffs.0.get(&Buff::ConveyerBelts) {
                        amount += *n as i32;
                    }
                    if let Some(n) = researched_buffs.0.get(&Buff::RoboticLoaders) {
                        amount += *n as i32;
                    }
                } else if gatherer.gatherer_type == GathererType::Oil {
                    if let Some(n) = researched_buffs.0.get(&Buff::AutomatedRefiners) {
                        amount += *n as i32;
                    }
                    if let Some(n) = researched_buffs.0.get(&Buff::Purifier) {
                        amount += *n as i32;
                    }
                } else if gatherer.gatherer_type == GathererType::Hydro {
                    if let Some(n) = researched_buffs.0.get(&Buff::ReinforcedTurbines) {
                        amount += *n as i32 * 2;
                    }
                } else if gatherer.gatherer_type == GathererType::Solar {
                    if let Some(n) = researched_buffs.0.get(&Buff::ImprovePanelTech) {
                        amount += *n as i32 * 2;
                    }
                }

                gathering_rate.add_to_resource_amount(&gatherer.gatherer_type, amount);

                let mut entity_node = node_storage.get_mut(entity).unwrap();
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
                entity_node.add(floating_text);
            }
            self.gathering_tick = 0.0;

            resources.increase_resource_for_gatherer_type(&GathererType::Coal, gathering_rate.coal);
            resources.increase_resource_for_gatherer_type(&GathererType::Oil, gathering_rate.oil);
            resources
                .increase_resource_for_gatherer_type(&GathererType::Solar, gathering_rate.solar);
            resources
                .increase_resource_for_gatherer_type(&GathererType::Hydro, gathering_rate.hydro);

            if resources.coal >= 50 {
                tutorial::next_step(
                    &entities,
                    &mut actions_storage,
                    &mut tutorial_step_storage,
                    &tutorial_ui_storage,
                    &node_storage,
                    TutorialStep::CoalGathered,
                    TutorialStep::SellResources,
                );
            } else if resources.coal > 0 {
                tutorial::next_step(
                    &entities,
                    &mut actions_storage,
                    &mut tutorial_step_storage,
                    &tutorial_ui_storage,
                    &node_storage,
                    TutorialStep::BuildCoal(0.0, 0.0),
                    TutorialStep::CoalGathered,
                );
            }
        }
    }
}
