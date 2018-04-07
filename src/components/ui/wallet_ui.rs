use specs::{Component, HashMapStorage};

pub struct WalletUI;

impl Component for WalletUI {
    type Storage = HashMapStorage<WalletUI>;
}
