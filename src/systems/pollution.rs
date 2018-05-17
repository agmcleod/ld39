use std::ops::DerefMut;
use specs::{Join, ReadStorage, System, Write, WriteStorage};
use components::{Gatherer, PollutionCount, Text, Wallet, ui::WalletUI};
use systems::{logic, FRAME_TIME};

pub struct Pollution {
    ticker: f32,
}

impl Pollution {
    pub fn new() -> Self {
        Pollution { ticker: 0.0 }
    }
}

impl<'a> System<'a> for Pollution {
    type SystemData = (
        ReadStorage<'a, Gatherer>,
        WriteStorage<'a, PollutionCount>,
        WriteStorage<'a, Text>,
        Write<'a, Wallet>,
        ReadStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            gatherer_storage,
            mut pollution_count_storage,
            mut text_storage,
            mut wallet_storage,
            wallet_ui_storage,
        ) = data;

        self.ticker += FRAME_TIME;

        if self.ticker < 1.0 {
            return;
        }

        self.ticker = 0.0;

        let mut pollution = 0;

        // can probably cache the pollution production amount on a tile when created
        // instead of calculating it here
        for gatherer in (&gatherer_storage).join() {
            pollution += gatherer.pollution;
        }

        if pollution > 0 {
            let wallet = wallet_storage.deref_mut();
            wallet.money -= pollution % 100 * 2;
            logic::update_text(
                format!("{}", wallet.money),
                &mut text_storage,
                &wallet_ui_storage,
            );

            for (pollution_count, text) in (&mut pollution_count_storage, &mut text_storage).join()
            {
                pollution_count.count = pollution;
                text.set_text(format!("Pollution: {}", pollution));
            }
        }
    }
}
