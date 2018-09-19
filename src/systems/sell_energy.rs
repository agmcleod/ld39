use components::ui::WalletUI;
use components::{ui::TutorialUI, upgrade::Buff, Actions, Button, ClickSound, DeltaTime, Input,
                 Node, PowerBar, ResearchedBuffs, ResourceType, Resources, Text, Transform,
                 TutorialStep, Wallet};
use entities::tutorial;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use systems::POWER_FACTOR;

pub struct SellEnergy {
    minute_ticker: f32,
}

impl SellEnergy {
    pub fn new() -> Self {
        SellEnergy { minute_ticker: 0.0 }
    }

    fn add_money<'a>(
        &mut self,
        amount: i32,
        wallet_storage: &mut Write<'a, Wallet>,
        wallet_ui_storage: &mut WriteStorage<'a, WalletUI>,
        text_storage: &mut WriteStorage<'a, Text>,
    ) {
        let wallet: &mut Wallet = wallet_storage.deref_mut();
        for (_, text) in (wallet_ui_storage, text_storage).join() {
            wallet.add_money(amount);
            text.set_text(format!("{}", wallet.money));
        }
    }
}

impl<'a> System<'a> for SellEnergy {
    type SystemData = (
        Entities<'a>,
        Write<'a, Actions>,
        WriteStorage<'a, Button>,
        Write<'a, ClickSound>,
        Read<'a, DeltaTime>,
        Read<'a, Input>,
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
            mut button_storage,
            mut click_sound_storage,
            delta_time_storage,
            input_storage,
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

        let click_sound: &mut ClickSound = click_sound_storage.deref_mut();

        let input: &Input = input_storage.deref();
        let resources: &mut Resources = resources_storage.deref_mut();

        let mut sell_button_clicked = false;
        for button in (&mut button_storage).join() {
            if button.name == "power_btn" && button.clicked(&input) {
                click_sound.play = true;
                sell_button_clicked = true;
            }
        }

        let researched_buffs = researched_buffs_storage.deref();

        if sell_button_clicked {
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

            self.add_money(
                power_to_spend,
                &mut wallet_storage,
                &mut wallet_ui_storage,
                &mut text_storage,
            );

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
        }

        if researched_buffs.0.contains(&Buff::SellPanelsToConsumers) {
            self.minute_ticker += delta_time_storage.deref().dt;
            if self.minute_ticker >= 1.0 {
                self.minute_ticker = 0.0;
                self.add_money(
                    50,
                    &mut wallet_storage,
                    &mut wallet_ui_storage,
                    &mut text_storage,
                );
            }
        }
    }
}
