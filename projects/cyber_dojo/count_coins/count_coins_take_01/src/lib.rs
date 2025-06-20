use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref COINS: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("quarters", 25);
        m.insert("dimes", 10);
        m.insert("nickels", 5);
        m.insert("pennies", 1);
        m
    };
}

/// CountCoins represents a coin counter that calculates ways to make change
pub struct CountCoins;

impl CountCoins {
    /// Creates a new CountCoins instance
    pub fn new() -> Self {
        CountCoins
    }

    /// Calculates the number of ways to make change for a given amount
    pub fn changes(&self, amount: i32) -> i32 {
        if amount < 1 {
            return 0;
        }

        let amount = amount as usize;
        let mut ways = vec![0; amount + 1];
        ways[0] = 1;

        for &coin in COINS.values() {
            let coin = coin as usize;
            for j in coin..=amount {
                ways[j] += ways[j - coin];
            }
        }

        ways[amount]
    }
}
