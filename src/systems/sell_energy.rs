use components::ui::WalletUI;
use components::{ui::TutorialUI, upgrade::Buff, Actions, DeltaTime, EntityLookup, Gatherer,
                 GathererType, GatheringRate, Node, PowerBar, ResearchedBuffs, ResourceType,
                 Resources, Text, Transform, TutorialStep, Wallet};
use entities::tutorial;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use systems::{logic, POWER_FACTOR, TICK_RATE};

pub struct SellEnergy {
    minute_ticker: f32,
    sell_ticker: f32,
}

impl SellEnergy {
    pub fn new() -> Self {
        SellEnergy {
            minute_ticker: 0.0,
            sell_ticker: 0.0,
        }
    }
}

impl<'a> System<'a> for SellEnergy {
    type SystemData = (
        Entities<'a>,
        Write<'a, Actions>,
        Read<'a, DeltaTime>,
        Read<'a, EntityLookup>,
        ReadStorage<'a, Gatherer>,
        Read<'a, GatheringRate>,
        WriteStorage<'a, Node>,
        WriteStorage<'a, PowerBar>,
        Read<'a, ResearchedBuffs>,
        Write<'a, Resources>,
        WriteStorage<'a, Text>,
        WriteStorage<'a, Transform>,
        Write<'a, TutorialStep>,
        ReadStorage<'a, TutorialUI>,
        Write<'a, Wallet>,
        WriteStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut actions_storage,
            delta_time_storage,
            entity_lookup_storage,
            gatherer_storage,
            gathering_rate_storage,
            node_storage,
            mut power_bar_storage,
            researched_buffs_storage,
            mut resources_storage,
            mut text_storage,
            mut transform_storage,
            mut tutorial_step_storage,
            tutorial_ui_storage,
            mut wallet_storage,
            mut wallet_ui_storage,
        ) = data;

        let resources: &mut Resources = resources_storage.deref_mut();

        let researched_buffs = researched_buffs_storage.deref();

        self.sell_ticker += delta_time_storage.deref().dt;

        if self.sell_ticker > TICK_RATE {
            self.sell_ticker = 0.0;
            // divide by power factor, so demand can be met based on resource numbers
            let mut amount_to_power = (&power_bar_storage).join().fold(0, |sum, power_bar| {
                PowerBar::get_max() - power_bar.power_left + sum
            }) / POWER_FACTOR;

            let mut power_to_spend = 0i32;

            tutorial::next_step(
                &entities,
                &mut actions_storage,
                &mut tutorial_step_storage,
                &tutorial_ui_storage,
                &node_storage,
                TutorialStep::SellResources,
                TutorialStep::ResourcesSold,
            );

            'resources: for r_type in &[
                ResourceType::Coal,
                ResourceType::Oil,
                ResourceType::Solar,
                ResourceType::Hydro,
            ] {
                loop {
                    let power = resources.withdraw_amount_for_type(*r_type, amount_to_power);

                    amount_to_power -= power;
                    power_to_spend += power;

                    // filled power requirement, exit top loop
                    if amount_to_power < r_type.get_efficiency_rate() {
                        break 'resources;
                    }

                    // ran out of this resource, break the infinite loop
                    if resources.get_amount_for_type(r_type) < r_type.get_efficiency_rate() {
                        break;
                    }
                }
            }

            if gathering_rate_storage.changed() {
                let entity = entity_lookup_storage.get("gathering_rate_coal").unwrap();
                {
                    let text = text_storage.get_mut(*entity).unwrap();
                    text.set_text(format!("Coal: {}", gathering_rate_storage.coal));
                }

                let entity = entity_lookup_storage.get("gathering_rate_oil").unwrap();
                {
                    let text = text_storage.get_mut(*entity).unwrap();
                    text.set_text(format!("Oil: {}", gathering_rate_storage.oil));
                }

                let entity = entity_lookup_storage.get("gathering_rate_hydro").unwrap();
                {
                    let text = text_storage.get_mut(*entity).unwrap();
                    text.set_text(format!("Hydro: {}", gathering_rate_storage.hydro));
                }

                let entity = entity_lookup_storage.get("gathering_rate_solar").unwrap();
                {
                    let text = text_storage.get_mut(*entity).unwrap();
                    text.set_text(format!("Solar: {}", gathering_rate_storage.solar));
                }

                let entity = entity_lookup_storage.get("gathering_rate_power").unwrap();
                {
                    let text = text_storage.get_mut(*entity).unwrap();
                    text.set_text(format!("Power: {}", power_to_spend));
                }
            }

            let money_from_power = power_to_spend;
            wallet_storage.add_money(money_from_power);
            power_to_spend *= POWER_FACTOR;

            for (transform, power_bar) in (&mut transform_storage, &mut power_bar_storage).join() {
                let amount_to_power = PowerBar::get_max() - power_bar.power_left;
                power_to_spend -= amount_to_power;
                if power_to_spend >= 0 {
                    power_bar.add_power(amount_to_power);
                // we do addition here since the number will be negative
                } else if amount_to_power + power_to_spend > 0 {
                    // add the larger number of amount to power
                    // (which was subtracted) by the negative value
                    // this will give us the amount left over
                    power_bar.add_power(amount_to_power + power_to_spend);
                }

                let width = PowerBar::get_max_width()
                    * (power_bar.power_left as f32 / PowerBar::get_max_f32());
                transform.size.x = width as u16;
            }

            let mut coal_pollution = 0;
            let mut oil_pollution = 0;
            let mut hydro_pollution = 0;
            let researched_buffs = researched_buffs_storage.deref();

            for gatherer in (&gatherer_storage).join() {
                let mut amount = gatherer.pollution;
                if amount > 0 {
                    if gatherer.gatherer_type == GathererType::Coal {
                        coal_pollution += amount;
                        if let Some(n) = researched_buffs.0.get(&Buff::ConveyerBelts) {
                            coal_pollution += *n as i32;
                        }
                    } else if gatherer.gatherer_type == GathererType::Oil {
                        oil_pollution += amount;
                        if let Some(n) = researched_buffs.0.get(&Buff::AutomatedRefiners) {
                            oil_pollution += 2 * *n as i32;
                        }
                    } else if gatherer.gatherer_type == GathererType::Hydro {
                        hydro_pollution += amount;
                    }
                }
            }

            if researched_buffs.0.contains_key(&Buff::PollutionFilters) {
                coal_pollution -= coal_pollution * 20 / 100;
            }
            if researched_buffs.0.contains_key(&Buff::FudgeTheNumbers) {
                oil_pollution -= oil_pollution * 20 / 100;
            }
            if researched_buffs.0.contains_key(&Buff::SalmonCannon) {
                hydro_pollution -= hydro_pollution * 20 / 100;
            }

            let pollution = coal_pollution + oil_pollution + hydro_pollution;

            let tax = if pollution > 0 {
                let tax = (money_from_power as f32 * (pollution as f32 / 100.0)) as i32;
                wallet_storage.remove_amount(tax);
                tax
            } else {
                0
            };

            // this could be potentially optimized by tracking last tax & money amounts in a resource, and check if it changes.
            // though gfx-glyph cache should do plenty for us
            let entity = entity_lookup_storage.get("gathering_rate_money").unwrap();
            {
                let text = text_storage.get_mut(*entity).unwrap();
                let sign = if tax > 0 { "-" } else { "" };
                text.set_text(format!(
                    "Income: ${}, Tax: {}${}",
                    money_from_power, sign, tax
                ));
            }

            logic::update_text_mut(
                format!("Wallet: ${}", wallet_storage.get_money()),
                &mut text_storage,
                &mut wallet_ui_storage,
            );
        }

        if researched_buffs
            .0
            .contains_key(&Buff::SellPanelsToConsumers)
        {
            self.minute_ticker += delta_time_storage.deref().dt;
            if self.minute_ticker >= 1.0 {
                self.minute_ticker = 0.0;
                wallet_storage.add_money(50);
                logic::update_text_mut(
                    format!("${}", wallet_storage.get_money()),
                    &mut text_storage,
                    &mut wallet_ui_storage,
                );
            }
        }
    }
}
