use std::ops::{Deref, DerefMut};
use specs::{Read, Write, Join, System, WriteStorage};
use components::{Button, ClickSound, Input, PowerBar, ResourceType, Resources, Text, Wallet};
use components::ui::WalletUI;

pub struct SellEnergy;

impl<'a> System<'a> for SellEnergy {
    type SystemData = (
        WriteStorage<'a, Button>,
        Write<'a, ClickSound>,
        Read<'a, Input>,
        WriteStorage<'a, PowerBar>,
        Write<'a, Resources>,
        WriteStorage<'a, Text>,
        Write<'a, Wallet>,
        WriteStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut button_storage,
            mut click_sound_storage,
            input_storage,
            mut power_bar_storage,
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

        if sell_button_clicked {
            let coal_amount = resources.withdraw_all_for_type(ResourceType::Coal) / 4;
            let oil_amount = resources.withdraw_all_for_type(ResourceType::Oil) / 3;
            let solar_amount = resources.withdraw_all_for_type(ResourceType::Solar) / 2;
            let hydro_amount = resources.withdraw_all_for_type(ResourceType::Hydro) / 2;
            for power_bar in (&mut power_bar_storage).join() {
                power_bar.add_power((coal_amount + oil_amount + solar_amount + hydro_amount) * 100);
            }

            let wallet: &mut Wallet = wallet_storage.deref_mut();

            for (_, text) in (&mut wallet_ui_storage, &mut text_storage).join() {
                wallet.add_money(coal_amount + oil_amount + solar_amount);
                text.set_text(format!("{}", wallet.money));
            }
        }
    }
}
