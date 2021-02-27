pub struct Merchant {
    accountant: std::sync::Arc<dyn agnostic::market::Accountant>,
    trader: std::sync::Arc<dyn agnostic::market::Trader>,
    sniffer: std::sync::Arc<dyn agnostic::market::Sniffer>,
}

impl Default for Merchant {
    fn default() -> Self {
        Merchant {
            accountant: std::sync::Arc::new(crate::accountant::Accountant {}),
            trader: std::sync::Arc::new(crate::trader::Trader {}),
            sniffer: std::sync::Arc::new(crate::sniffer::Sniffer {}),
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
