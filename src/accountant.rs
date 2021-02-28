pub struct Balance {
    pub first_coin_amount: f64,
    pub second_coin_amount: f64,
}

pub struct Accountant {
    balance: Balance
}

impl Default for Accountant {
    fn default() -> Accountant {
        Accountant {
            balance: Balance {
                first_coin_amount: 10f64,
                second_coin_amount: 10f64,
            },
        }
    }
}

impl agnostic::market::Accountant for Accountant {
    fn ask(
        &self,
        coin: agnostic::trading_pair::Coin
    ) -> agnostic::market::Future<Result<agnostic::currency::Currency, String>> {
        let amount = self.balance.first_coin_amount;
        Box::pin(async move {
            Ok(agnostic::currency::Currency {
                coin,
                held: amount,
                amount,
            })
        })
    }

    fn ask_both(
        &self,
        first_coin: agnostic::trading_pair::Coin,
        second_coin: agnostic::trading_pair::Coin,
    ) -> agnostic::market::Future<Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>> {
        let (first_coin_amount, second_coin_amount) = (
            self.balance.first_coin_amount,
            self.balance.second_coin_amount,
        );
        Box::pin(async move {
            Ok((
                agnostic::currency::Currency {
                    coin: first_coin,
                    held: first_coin_amount,
                    amount: first_coin_amount,
                },
                agnostic::currency::Currency {
                    coin: second_coin,
                    held: second_coin_amount,
                    amount: second_coin_amount,
                },
            ))
        })
    }

    fn calculate_volume(
        &self,
        _trading_pair: agnostic::trading_pair::TradingPair,
        price: f64,
        amount: f64) -> f64 {
        price * amount
    }

    fn nearest_price(&self, _trading_pair: agnostic::trading_pair::TradingPair, price: f64) -> f64 {
        price
    }
}
