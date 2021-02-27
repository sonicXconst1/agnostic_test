pub struct Accountant {
}

impl agnostic::market::Accountant for Accountant {
    fn ask(
        &self,
        coin: agnostic::trading_pair::Coin
    ) -> agnostic::market::Future<Result<agnostic::currency::Currency, String>> {
        Box::pin(async move {
            Ok(agnostic::currency::Currency {
                coin,
                held: 2f64,
                amount: 2f64,
            })
        })
    }

    fn ask_both(
        &self,
        first_coin: agnostic::trading_pair::Coin,
        second_coin: agnostic::trading_pair::Coin,
    ) -> agnostic::market::Future<Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>> {
        Box::pin(async move {
            Ok((
                agnostic::currency::Currency {
                    coin: first_coin,
                    held: 2f64,
                    amount: 2f64,
                },
                agnostic::currency::Currency {
                    coin: second_coin,
                    held: 4f64,
                    amount: 4f64,
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
