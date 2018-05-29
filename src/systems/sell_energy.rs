use std::ops::{Deref, DerefMut};
use specs::{Join, Read, System, Write, WriteStorage};
use components::{Button, ClickSound, DeltaTime, Input, PowerBar, ResearchedBuffs, ResourceType, Resources,
                 Text, Wallet, upgrade::Buff};
use components::ui::WalletUI;

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
        WriteStorage<'a, Button>,
        Write<'a, ClickSound>,
        Read<'a, DeltaTime>,
        Read<'a, Input>,
        WriteStorage<'a, PowerBar>,
        Read<'a, ResearchedBuffs>,
        Write<'a, Resources>,
        WriteStorage<'a, Text>,
        Write<'a, Wallet>,
        WriteStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut button_storage,
            mut click_sound_storage,
            delta_time_storage,
            input_storage,
            mut power_bar_storage,
            researched_buffs_storage,
            mut resources_storage,
            mut text_storage,
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
            let mut coal_amount = resources.withdraw_all_for_type(ResourceType::Coal) / 4;
            if researched_buffs.0.contains(&Buff::Purifier) {
                coal_amount += coal_amount * 5 / 100;
            }

            let oil_amount = resources.withdraw_all_for_type(ResourceType::Oil) / 3;
            let solar_amount = resources.withdraw_all_for_type(ResourceType::Solar) / 2;
            let hydro_amount = resources.withdraw_all_for_type(ResourceType::Hydro) / 2;
            for power_bar in (&mut power_bar_storage).join() {
                power_bar.add_power((coal_amount + oil_amount + solar_amount + hydro_amount) * 100);
            }

            self.add_money(
                coal_amount + oil_amount + solar_amount,
                &mut wallet_storage,
                &mut wallet_ui_storage,
                &mut text_storage,
            );
        }

        if researched_buffs.0.contains(&Buff::SellPanelsToConsumers) {
            self.minute_ticker += delta_time_storage.deref().dt;
            if self.minute_ticker >= 1.0 {
                self.minute_ticker = 0.0;
                self.add_money(
                    10,
                    &mut wallet_storage,
                    &mut wallet_ui_storage,
                    &mut text_storage,
                );
            }
        }
    }
}
