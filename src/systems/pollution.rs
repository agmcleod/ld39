use components::{ui::{PollutionCount, WalletUI},
                 upgrade::Buff,
                 DeltaTime,
                 Gatherer,
                 GathererType,
                 ResearchedBuffs,
                 Text,
                 Wallet};
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};
use std::ops::{Deref, DerefMut};
use systems::logic;

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
        Read<'a, DeltaTime>,
        ReadStorage<'a, Gatherer>,
        WriteStorage<'a, PollutionCount>,
        Read<'a, ResearchedBuffs>,
        WriteStorage<'a, Text>,
        Write<'a, Wallet>,
        ReadStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            delta_time_storage,
            gatherer_storage,
            mut pollution_count_storage,
            researched_buffs_storage,
            mut text_storage,
            mut wallet_storage,
            wallet_ui_storage,
        ) = data;

        self.ticker += delta_time_storage.deref().dt;

        if self.ticker < 1.0 {
            return;
        }

        self.ticker = 0.0;

        let mut coal_pollution = 0;
        let mut oil_pollution = 0;
        let mut hydro_pollution = 0;
        let researched_buffs = researched_buffs_storage.deref();

        for gatherer in (&gatherer_storage).join() {
            let mut amount = gatherer.pollution;
            if amount > 0 {
                if gatherer.gatherer_type == GathererType::Coal {
                    coal_pollution += amount;
                    if researched_buffs.0.contains(&Buff::ConveyerBelts) {
                        coal_pollution += 1;
                    }
                } else if gatherer.gatherer_type == GathererType::Oil {
                    oil_pollution += amount;
                    if researched_buffs.0.contains(&Buff::AutomatedRefiners) {
                        oil_pollution += 1;
                    }
                } else if gatherer.gatherer_type == GathererType::Hydro {
                    hydro_pollution += amount;
                }
            }
        }

        if researched_buffs.0.contains(&Buff::PollutionFilters) {
            coal_pollution -= coal_pollution * 20 / 100;
        }
        if researched_buffs.0.contains(&Buff::FudgeTheNumbers) {
            oil_pollution -= oil_pollution * 20 / 100;
        }
        if researched_buffs.0.contains(&Buff::SalmonCannon) {
            hydro_pollution -= hydro_pollution * 20 / 100;
        }

        let pollution = coal_pollution + oil_pollution + hydro_pollution;

        if pollution > 0 {
            let wallet = wallet_storage.deref_mut();
            wallet.money -= pollution;
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
