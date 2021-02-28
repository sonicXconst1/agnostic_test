#[derive(Clone, Debug)]
pub struct Order {
    pub price: f64,
    pub amount: f64,
}

pub struct Sniffer {
    pub orders: Vec<Order>
}

impl Default for Sniffer {
    fn default() -> Sniffer {
        Sniffer {
            orders: vec![
                Order {
                    price: 4f64,
                    amount: 4f64,
                },
                Order {
                    price: 4f64,
                    amount: 4f64,
                },
                Order {
                    price: 4f64,
                    amount: 4f64,
                },
            ],
        }
    }
}

impl agnostic::market::Sniffer for Sniffer {
    fn all_the_best_orders(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
        _count: u32,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::Order>, String>> {
        let orders = self.orders.clone();
        Box::pin(async move {
            Ok(orders.into_iter()
               .map(|order| agnostic::order::Order {
                   trading_pair: trading_pair.clone(),
                   price: order.price,
                   amount: order.amount,
               })
               .collect()
            )
        })
    }

    fn the_best_order(&self, trading_pair: agnostic::trading_pair::TradingPair) -> agnostic::market::Future<Result<agnostic::order::Order, String>> {
        let first_order = self.orders.get(0).map(|order| order.clone());
        Box::pin(async move {
            match first_order {
                Some(order) => Ok(agnostic::order::Order {
                    trading_pair: trading_pair.clone(),
                    price: order.price,
                    amount: order.amount,
                }),
                None => Err("0 orders on the market".to_owned()),
            }
        })
    }

    fn get_my_orders(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::OrderWithId>, String>> {
        let orders = self.orders.clone();
        Box::pin(async move {
            Ok(orders
               .into_iter()
               .enumerate()
               .map(|(index, order)| agnostic::order::OrderWithId {
                   id: index.to_string(),
                   trading_pair: trading_pair.clone(),
                   amount: order.amount,
                   price: order.price,
                })
               .collect()
            )
        })
    }
}
