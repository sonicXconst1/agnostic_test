use agnostic::market::{Accountant, Sniffer, Trader};
use agnostic::trading_pair::Side;
use std::sync::Arc;
use crate::sniffer::StockGenerator;

pub struct Merchant {
    accountant: std::sync::Arc<dyn agnostic::market::Accountant>,
    trader: std::sync::Arc<dyn agnostic::market::Trader>,
    sniffer: std::sync::Arc<dyn agnostic::market::Sniffer>,
}

impl Merchant {
    pub fn with_sniffer(sniffer: Arc<dyn Sniffer>) -> Merchant {
        let accountant = Arc::new(crate::accountant::Accountant::default());
        let trader = Arc::new(crate::trader::Trader::default());
        Merchant {
            sniffer,
            trader,
            accountant
        }
    }

    pub fn custom(
        accountant: Arc<dyn Accountant>,
        sniffer: Arc<dyn Sniffer>,
        trader: Arc<dyn Trader>,
    ) -> Merchant {
        Merchant {
            accountant,
            sniffer,
            trader,
        }
    }
}

impl Default for Merchant {
    fn default() -> Self {
        let base_price = 0.5;
        let price_step = 0.001;
        let orders_count = 10;
        let sell_stock_generator = StockGenerator::new(
            Side::Sell,
            base_price + price_step,
            price_step,
            orders_count);
        let buy_stock_generator = StockGenerator::new(
            Side::Buy,
            base_price - price_step,
            price_step,
            orders_count);
        Merchant {
            accountant: Arc::new(crate::accountant::Accountant::default()),
            trader: Arc::new(crate::trader::Trader::default()),
            sniffer: Arc::new(crate::sniffer::Sniffer::fixed_amount(
                    sell_stock_generator,
                    buy_stock_generator,
                    100f64))
        }
    }
}

impl agnostic::merchant::Merchant for Merchant {
    fn accountant(&self) -> std::sync::Arc<dyn agnostic::market::Accountant> {
        self.accountant.clone()
    }

    fn trader(&self) -> std::sync::Arc<dyn agnostic::market::Trader> {
        self.trader.clone()
    }

    fn sniffer(&self) -> std::sync::Arc<dyn agnostic::market::Sniffer> {
        self.sniffer.clone()
    }
}

#[cfg(test)]
mod test {
    use agnostic::merchant::Merchant;

    #[test]
    fn default_merchant() {
        let merchant = super::Merchant::default();
        crate::sniffer::test::test_sniffer(
            merchant.sniffer().as_ref(),
            0.501,
            0.510,
            0.499,
            0.490,
            10);
    }
}
