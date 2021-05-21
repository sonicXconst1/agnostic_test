use agnostic::trade::{Trade, TradeResult};
use agnostic::order::{Order, OrderWithId};
use agnostic::market;
use agnostic::trading_pair::Target;

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
    pub trades: Vec<Trade>
}

impl TradesLogger {
    pub fn new(trader: Trader) -> Self {
        TradesLogger {
            trader,
            ..Default::default()
        }
    }

    pub async fn create_order(&mut self, order: Order) {
    }
}

impl agnostic::market::Trader for TradesLogger {
    fn create_order(&self, order: Order) -> market::Future<Result<Trade, String>> {
        let result = self.trader.create_order(order);
        let result = async move {
            let s = result.await;
            s
        };
        Box::pin(result)
    }

    fn delete_order(&self, id: &str) -> market::Future<Result<(), String>> {
        todo!()
    }
}
