const START_AMOUNT: i32 = 25;

#[derive(Default)]
pub struct Wallet {
    money: i32,
    pub last_credit: i32,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            money: START_AMOUNT,
            last_credit: START_AMOUNT,
        }
    }

    pub fn start_amount() -> i32 {
        START_AMOUNT
    }

    pub fn add_money(&mut self, amount: i32) {
        self.money += amount;
        self.last_credit = amount;
    }

    pub fn get_money(&self) -> i32 {
        self.money
    }

    pub fn remove_amount(&mut self, amount: i32) {
        self.money -= amount;
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
