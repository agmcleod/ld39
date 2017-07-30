use std::ops::{Deref, DerefMut};
use specs::{Fetch, FetchMut, Join, ReadStorage, WriteStorage, System};
use components::{Input, Button, PowerBar, Resources};

pub struct SellEnergy;

impl<'a> System<'a> for SellEnergy {
    type SystemData = (
        WriteStorage<'a, Button>,
        Fetch<'a, Input>,
        WriteStorage<'a, PowerBar>,
        FetchMut<'a, Resources>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut button_storage, input_storage, mut power_bar_storage, mut resources_storage) = data;

        let input: &Input = input_storage.deref();
        let resources: &mut Resources = resources_storage.deref_mut();

        let mut sell_button_clicked = false;
        for button in (&mut button_storage).join() {
            if button.name == "sell".to_string() && button.clicked(&input) {
                sell_button_clicked = true;
            }
        }

        if sell_button_clicked {
            let amount = resources.get_resources(20);
            for power_bar in (&mut power_bar_storage).join() {
                power_bar.add_power(amount);
            }
        }
    }
}