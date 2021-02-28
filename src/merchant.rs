pub struct Merchant {
    accountant: std::sync::Arc<dyn agnostic::market::Accountant>,
    trader: std::sync::Arc<dyn agnostic::market::Trader>,
    sniffer: std::sync::Arc<dyn agnostic::market::Sniffer>,
}

impl Merchant {
    pub fn with_orders(price: f64, amount: f64) -> Merchant {
        let sniffer = std::sync::Arc::new(crate::sniffer::Sniffer {
            orders: vec![crate::sniffer::Order { price, amount}; 3],
        });
        let accountant = std::sync::Arc::new(crate::accountant::Accountant::default());
        let trader = std::sync::Arc::new(crate::trader::Trader::default());
        Merchant {
            sniffer,
            trader,
            accountant
        }
    }
}

impl Default for Merchant {
    fn default() -> Self {
        Merchant {
            accountant: std::sync::Arc::new(crate::accountant::Accountant::default()),
            trader: std::sync::Arc::new(crate::trader::Trader::default()),
            sniffer: std::sync::Arc::new(crate::sniffer::Sniffer::default()),
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
