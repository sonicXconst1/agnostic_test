use agnostic::trade::{Trade, TradeResult};
use agnostic::order::{Order, OrderWithId};
use agnostic::market;
use agnostic::trading_pair::Target;
use std::sync::{
    Arc,
    Mutex
};
use rand;

#[derive(Debug, Default)]
pub struct Trader {
}

fn get_id() -> String {
    rand::random::<u32>().to_string()
}

impl agnostic::market::Trader for Trader {
    fn create_order(&self, order: Order) -> market::Future<Result<Trade, String>> {
        Box::pin(async move {
            log::debug!("Creating order {:#?}", order);
            match order.trading_pair.target {
                Target::Market => Ok(Trade::Market(TradeResult {
                    id: get_id(),
                    trading_pair: order.trading_pair,
                    price: order.price,
                    amount: order.amount,
                })),
                Target::Limit => Ok(Trade::Limit(OrderWithId {
                    id: get_id(),
                    trading_pair: order.trading_pair,
                    price: order.price,
                    amount: order.amount,
                })),
            }
        })
    }

    fn delete_order(&self, _id: &str) -> agnostic::market::Future<Result<(), String>> {
        Box::pin(async {
            Ok(())
        })
    }
}

#[derive(Default, Debug)]
pub struct TradesLogger {
    trader: Trader,
    pub create_order_log: Arc<Mutex<Vec<Result<Trade, String>>>>,
    pub delete_order_log: Arc<Mutex<Vec<Result<(), String>>>>,

}

impl TradesLogger {
    pub fn new(trader: Trader) -> Self {
        TradesLogger {
            trader,
            ..Default::default()
        }
    }

    pub fn with_orders(trader: Trader, create_order_results: Vec<Trade>) -> TradesLogger {
        TradesLogger {
            trader,
            create_order_log: Arc::new(Mutex::new(
                create_order_results.into_iter().map(|item| Ok(item)).collect()
            )),
            ..Default::default()
        }
    }
}

impl agnostic::market::Trader for TradesLogger {
    fn create_order(&self, order: Order) -> market::Future<Result<Trade, String>> {
        let trade = self.trader.create_order(order);
        let create_order_log = self.create_order_log.clone();
        Box::pin(async move {
            let trade = trade.await;
            create_order_log.lock().unwrap().push(trade.clone());
            trade
        })
    }

    fn delete_order(&self, id: &str) -> market::Future<Result<(), String>> {
        self.trader.delete_order(id)
    }
}

impl From<Trader> for TradesLogger {
    fn from(trader: Trader) -> Self {
        TradesLogger {
            trader,
            ..Default::default()
        }
    }
}

impl From<TradesLogger> for Trader {
    fn from(logger: TradesLogger) -> Trader {
        logger.trader
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use agnostic::trading_pair::{
        Target,
        Side,
        TradingPair,
        Coins
    };
    use agnostic::market::Trader as TraderTrait;

    #[test]
    fn test_trades_logger() {
        let trader = TradesLogger::default();
        let price = 100f64;
        let amount = 100f64;
        let order = Order {
            trading_pair: TradingPair {
                coins: Coins::TonUsdt,
                side: Side::Sell,
                target: Target::Market,
            },
            price,
            amount,
        };
        let mut ok_counter = 0;
        let mut error_counter = 0;
        for _ in 0..5 {
            match tokio_test::block_on(trader.create_order(order.clone())) {
                Ok(_trade) => ok_counter += 1,
                Err(_error) => error_counter += 1,
            };
        }
        assert_eq!(
            trader.create_order_log.lock().unwrap().iter().filter(|item| item.is_ok()).count(),
            ok_counter,
        );
        assert_eq!(
            trader.create_order_log.lock().unwrap().iter().filter(|item| item.is_err()).count(),
            error_counter,
        );
    }
}
