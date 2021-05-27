use agnostic::trade::{Trade, TradeResult};
use agnostic::order::{Order, OrderWithId};
use agnostic::market;
use agnostic::trading_pair::Target;
use std::sync::{
    Arc,
    Mutex
};

#[derive(Debug, Default)]
pub struct Trader {
    pub created_orders: Vec<Trade>,
}

impl agnostic::market::Trader for Trader {
    fn create_order(&self, order: Order) -> market::Future<Result<Trade, String>> {
        Box::pin(async move {
            log::debug!("Creating order {:#?}", order);
            match order.trading_pair.target {
                Target::Market => Ok(Trade::Market(TradeResult {
                    id: "1337".to_owned(),
                    trading_pair: order.trading_pair,
                    price: order.price,
                    amount: order.amount,
                })),
                Target::Limit => Ok(Trade::Limit(OrderWithId {
                    id: "1337".to_owned(),
                    trading_pair: order.trading_pair,
                    price: order.price,
                    amount: order.amount,
                })),
            }
        })
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        let id = id.to_owned();
        Box::pin(async move {
            log::debug!("Deleting order with id: {:#?}", id);
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
        let delete_future = self.trader.delete_order(id);
        let delete_order_log = self.delete_order_log.clone();
        Box::pin(async move {
            let delete_result = delete_future.await;
            delete_order_log.lock().unwrap().push(delete_result.clone());
            delete_result
        })
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
