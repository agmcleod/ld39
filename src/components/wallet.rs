const START_AMOUNT: usize = 20;

pub struct Wallet {
    pub money: usize,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet{
            money: START_AMOUNT,
        }
    }

    pub fn start_amount() -> usize {
        START_AMOUNT
    }

    pub fn add_money(&mut self, amount: usize) {
        self.money += amount;
    }

    pub fn reset(&mut self) {
        self.money = START_AMOUNT;
    }

    pub fn spend(&mut self, amount: usize) -> bool {
        if amount > self.money {
            false
        } else {
            self.money -= amount;
            true
        }
    }
}
