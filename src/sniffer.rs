use agnostic::trading_pair::{Side, Target};

#[derive(Default, Debug)]
pub struct SnifferBuilder {
    buy_generator: Option<StockGenerator>,
    sell_generator: Option<StockGenerator>,
    my_orders: Option<Vec<OrderWithId>>,
}

impl SnifferBuilder {
    pub fn new() -> Self {
        SnifferBuilder {
            ..Default::default()
        }
    }

    pub fn buy_stock_generator(self, generator: StockGenerator) -> Self {
        Self {
            buy_generator: Some(generator),
            sell_generator: self.sell_generator,
            my_orders: self.my_orders,
        }
    }

    pub fn sell_stock_generator(self, generator: StockGenerator) -> Self {
        Self {
            buy_generator: self.buy_generator,
            sell_generator: Some(generator),
            my_orders: self.my_orders,
        }
    }

    pub fn my_orders(self, my_orders: Vec<OrderWithId>) -> Self {
        Self {
            buy_generator: self.buy_generator,
            sell_generator: self.sell_generator,
            my_orders: Some(my_orders),
        }
    }

    pub fn build(self, amount: f64) -> Sniffer {
        fn create_default_generator(side: Side) -> StockGenerator {
            StockGenerator {
                base_price: 1f64,
                step: match side {
                    Side::Sell => 0.1f64,
                    Side::Buy => -0.1f64,
                },
                count: 3, 
            }
        }
        fn create_orders(amount: f64, side: Side, generator: Option<StockGenerator>) -> Vec<Order> {
            generator
                .unwrap_or(create_default_generator(side))
                .generate_orders(amount)
        }
        Sniffer {
            sell_orders: create_orders(amount, Side::Sell, self.sell_generator),
            buy_orders: create_orders(amount, Side::Buy, self.buy_generator),
            my_orders: self.my_orders.unwrap_or(Default::default()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Order {
    pub price: f64,
    pub amount: f64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct OrderWithId {
    pub id: String,
    pub price: f64,
    pub amount: f64,
}

#[derive(Default)]
pub struct Sniffer {
    pub sell_orders: Vec<Order>,
    pub buy_orders: Vec<Order>,
    pub my_orders: Vec<OrderWithId>,
}

#[derive(Debug)]
pub struct StockGenerator {
    base_price: f64,
    step: f64,
    count: usize
}

impl StockGenerator {
    pub fn new(
        side: Side,
        base_price: f64,
        step: f64,
        count: usize) -> Self {
        StockGenerator {
            base_price,
            step: match side {
                Side::Buy => -step,
                Side::Sell => step,
            },
            count,
        }
    }
}

impl StockGenerator {
    pub fn generate_orders(&self, amount: f64) -> Vec<Order> {
        (0..self.count)
            .map(|step_index| {
                let price = self.base_price + (self.step * (step_index as f64));
                Order {
                    price,
                    amount,
                }
            })
            .collect()
    }
}

impl Sniffer {
    pub fn fixed_amount(sell: StockGenerator, buy: StockGenerator, amount: f64) -> Sniffer {
        Sniffer {
            sell_orders: sell.generate_orders(amount),
            buy_orders: buy.generate_orders(amount),
            my_orders: Vec::new()
        }
    }
}

impl agnostic::market::Sniffer for Sniffer {
    fn all_the_best_orders(
        &self,
        trading_pair: agnostic::trading_pair::TradingPair,
        count: u32,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::Order>, String>> {
        let orders = match (trading_pair.target, trading_pair.side) {
            (Target::Market, Side::Sell) => self.buy_orders.clone(),
            (Target::Market, Side::Buy) => self.sell_orders.clone(),
            (Target::Limit, Side::Sell) => self.sell_orders.clone(),
            (Target::Limit, Side::Buy) => self.buy_orders.clone(),
        };
        Box::pin(async move {
            Ok(orders.into_iter()
               .map(|order| agnostic::order::Order {
                   trading_pair: trading_pair.clone(),
                   price: order.price,
                   amount: order.amount,
               })
               .take(count as usize)
               .collect()
            )
        })
    }

    fn get_my_orders(
        &self,
        _trading_pair: agnostic::trading_pair::TradingPair,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::OrderWithId>, String>> {
        Box::pin(async move {
            Ok(Vec::new())
        })
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use agnostic::trading_pair::{TradingPair, Side, Coins, Target};
    use agnostic::market::Sniffer;
    use utils::almost_equal;

    #[test]
    fn empty_stock_generator() {
        let stock_generator = StockGenerator::new(Side::Sell, 0f64, 0f64, 0);
        let orders = stock_generator.generate_orders(100f64);
        assert_eq!(orders.len(), 0)
    }

    #[test]
    fn stock_generator_default() {
        let base_price = 1f64;
        let step = 0.1f64;
        let count = 10;
        let amount = 100f64;
        let stock_generator = StockGenerator::new(Side::Sell, base_price, step, count);
        let orders = stock_generator.generate_orders(amount);
        assert_eq!(orders.len(), count);
        let first_price = orders.first().unwrap().price;
        assert!(almost_equal(first_price, base_price));
        let last_price = orders.last().unwrap().price;
        assert!(almost_equal(last_price, 1.9f64)); 
        let stock_generator = StockGenerator::new(Side::Buy, base_price, step, count);
        let orders = stock_generator.generate_orders(amount);
        assert_eq!(orders.len(), count);
        let first_price = orders.first().unwrap().price;
        assert!(almost_equal(first_price, base_price));
        let last_price = orders.last().unwrap().price;
        assert!(almost_equal(last_price, 0.1f64)); 
    }

    #[test]
    fn sniffer() {
        let base_price = 1f64;
        let step = 0.1f64;
        let count = 10;
        let amount = 100f64;
        let sell_stock_generator = StockGenerator::new(Side::Sell, base_price, step, count);
        let buy_stock_generator = StockGenerator::new(Side::Buy, base_price, step, count);
        let sniffer = super::Sniffer::fixed_amount(sell_stock_generator, buy_stock_generator, amount);
        test_sniffer(&sniffer, base_price, 1.9f64, base_price, 0.1f64, count)
    }

    pub(crate) fn test_sniffer(
        sniffer: &dyn Sniffer,
        expected_first_sell_price: f64,
        expected_last_sell_price: f64,
        expected_first_buy_price: f64,
        expected_last_buy_price: f64,
        expected_count: usize
    ) {
        test_sniffer_side(
            sniffer,
            Side::Sell,
            expected_first_sell_price, 
            expected_last_sell_price,
            expected_count);
        test_sniffer_side(
            sniffer,
            Side::Buy,
            expected_first_buy_price, 
            expected_last_buy_price,
            expected_count);
    }

    fn test_sniffer_side(
        sniffer: &dyn Sniffer,
        side: Side,
        expected_first_price: f64,
        expected_last_price: f64,
        expected_count: usize,
    ) {
        let count = (expected_count * 2) as u32;
        let trading_pair = TradingPair {
            coins: Coins::TonUsdt,
            side,
            target: Target::Limit,
        };
        let my_orders = sniffer.get_my_orders(trading_pair.clone());
        let my_orders = tokio_test::block_on(my_orders).unwrap();
        assert_eq!(my_orders.len(), 0);
        let all_the_best_orders = sniffer.all_the_best_orders(trading_pair.clone(), count);
        let all_the_best_orders = tokio_test::block_on(all_the_best_orders).unwrap();
        assert_eq!(all_the_best_orders.len(), expected_count);
        if expected_count == 0 {
            return;
        }
        let first_order = all_the_best_orders.first().unwrap();
        assert!(almost_equal(first_order.price, expected_first_price));
        let last_order = all_the_best_orders.last().unwrap();
        assert!(almost_equal(last_order.price, expected_last_price), "Last sell order: {:#?}", last_order);
    }
}
