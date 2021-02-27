pub struct Sniffer {
}

impl agnostic::market::Sniffer for Sniffer {
    fn all_the_best_orders(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
        count: u32,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::Order>, String>> {
        Box::pin(async move {
            Ok(vec![
                agnostic::order::Order {
                    trading_pair: trading_pair.clone(),
                    price: 1f64,
                    amount: 1f64,
                }; 
                count as usize
            ])
        })
    }

    fn the_best_order(&self, trading_pair: agnostic::trading_pair::TradingPair) -> agnostic::market::Future<Result<agnostic::order::Order, String>> {
        Box::pin(async move {
            Ok(agnostic::order::Order {
                trading_pair: trading_pair.clone(),
                price: 2f64,
                amount: 2f64,
            })
        })
    }

    fn get_my_orders(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::OrderWithId>, String>> {
        Box::pin(async move {
            Ok(vec![
                agnostic::order::OrderWithId {
                    id: "1".to_owned(),
                    trading_pair: trading_pair.clone(),
                    price: 1f64,
                    amount: 1f64,
                },
                agnostic::order::OrderWithId {
                    id: "2".to_owned(),
                    trading_pair: trading_pair.clone(),
                    price: 2f64,
                    amount: 2f64,
                },
                agnostic::order::OrderWithId {
                    id: "3".to_owned(),
                    trading_pair: trading_pair.clone(),
                    price: 3f64,
                    amount: 3f64,
                },
            ])
        })
    }
}
