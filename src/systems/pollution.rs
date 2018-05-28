use std::ops::{Deref, DerefMut};
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};
use components::{Gatherer, GathererType, ResearchedBuffs, Text, Wallet,
                 ui::{PollutionCount, WalletUI}, upgrade::Buff};
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
        Read<'a, ResearchedBuffs>,
        WriteStorage<'a, Text>,
        Write<'a, Wallet>,
        ReadStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            gatherer_storage,
            mut pollution_count_storage,
            researched_buffs_storage,
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
        let researched_buffs = researched_buffs_storage.deref();

        for gatherer in (&gatherer_storage).join() {
            let mut amount = gatherer.pollution;
            if amount > 0 {
                if gatherer.gatherer_type == GathererType::Coal {
                    if researched_buffs.0.contains(&Buff::ConveyerBelts) {
                        amount += 1;
                    } else if researched_buffs.0.contains(&Buff::PollutionFilters) {
                        amount -= amount * 20 / 100;
                    }
                } else if gatherer.gatherer_type == GathererType::Oil {
                    if researched_buffs.0.contains(&Buff::AutomatedRefiners) {
                        amount += 1;
                    } else if researched_buffs.0.contains(&Buff::FudgeTheNumbers) {
                        amount -= amount * 20 / 100;
                    }
                } else if gatherer.gatherer_type == GathererType::Hydro {
                    if researched_buffs.0.contains(&Buff::SalmonCannon) {
                        amount -= amount * 20 / 100;
                    }
                }
            }

            pollution += amount;
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
