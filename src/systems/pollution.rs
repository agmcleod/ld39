use std::ops::{Deref, DerefMut};
use specs::{Fetch, FetchMut, Join, ReadStorage, System, WriteStorage};
use components::{Gatherer, GathererType, PollutionCount, ProtectedNodes, Text, Tile, TileType, Transform, Wallet, ui::{WalletUI}};
use systems::{FRAME_TIME, logic};

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
        Fetch<'a, ProtectedNodes>,
        WriteStorage<'a, Text>,
        ReadStorage<'a, Transform>,
        FetchMut<'a, Wallet>,
        ReadStorage<'a, WalletUI>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gatherer_storage, mut pollution_count_storage, protected_nodes_storage, mut text_storage, transform_storage, mut wallet_storage, wallet_ui_storage) = data;

        self.ticker += FRAME_TIME;

        if self.ticker < 1.0 {
            return;
        }

        self.ticker = 0.0;

        let protected_nodes = protected_nodes_storage.deref();

        let mut pollution = 0;

        // can probably cache the pollution production amount on a tile when created
        // instead of calculating it here
        for (gatherer, transform) in (&gatherer_storage, &transform_storage).join() {
            if gatherer.polluting {
                let col = (transform.get_pos().x / Tile::get_size()) as i32;
                let row = (transform.get_pos().y / Tile::get_size()) as i32;

                for i in -1..2 {
                    for j in -1..2 {
                        if let Some(tile_type) = protected_nodes.nodes.get(&(col + i, row + i)) {
                            if gatherer.gatherer_type != GathererType::Hydro
                                || (gatherer.gatherer_type == GathererType::Hydro
                                    && (*tile_type == TileType::EcoSystem
                                        || *tile_type == TileType::River))
                            {
                                pollution += gatherer.gatherer_type.get_pollution_amount();
                            }
                        }
                    }
                }
            }
        }

        if pollution > 0 {
            let wallet = wallet_storage.deref_mut();
            wallet.money -= pollution / 10;
            logic::update_text(
                format!("{}", wallet.money),
                &mut text_storage,
                &wallet_ui_storage,
            );

            for (pollution_count, text) in (&mut pollution_count_storage, &mut text_storage).join() {
                pollution_count.count = pollution;
                text.set_text(format!("Pollution: {}", pollution));
            }
        }
    }
}
