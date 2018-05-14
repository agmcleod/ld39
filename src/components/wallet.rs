const START_AMOUNT: i32 = 20;

#[derive(Default)]
pub struct Wallet {
    pub money: i32,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            money: START_AMOUNT,
        }
    }

    pub fn start_amount() -> i32 {
        START_AMOUNT
    }

    pub fn add_money(&mut self, amount: i32) {
        self.money += amount;
    }

    pub fn reset(&mut self) {
        self.money = START_AMOUNT;
    }

    pub fn spend(&mut self, amount: i32) -> bool {
        if amount > self.money {
            false
        } else {
            self.money -= amount;
            true
        }
    }
}
