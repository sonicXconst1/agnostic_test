use agnostic::market::{Accountant, Sniffer, Trader};
use std::sync::Arc;

pub struct Merchant {
    id: &'static str,
    accountant: std::sync::Arc<dyn agnostic::market::Accountant>,
    trader: std::sync::Arc<dyn agnostic::market::Trader>,
    sniffer: std::sync::Arc<dyn agnostic::market::Sniffer>,
}

impl Merchant {
    pub fn with_sniffer(id: &'static str, sniffer: Arc<dyn Sniffer>) -> Merchant {
        let accountant = Arc::new(crate::accountant::Accountant::default());
        let trader = Arc::new(crate::trader::Trader::default());
        Merchant {
            id,
            sniffer,
            trader,
            accountant
        }
    }

    pub fn with_trader(id: &'static str, trader: Arc<dyn Trader>) -> Merchant {
        let accountant = Arc::new(crate::accountant::Accountant::default());
        let sniffer = Arc::new(crate::sniffer::Sniffer::default());
        Merchant {
            id,
            sniffer,
            trader,
            accountant
        }
    }

    pub fn custom(
        id: &'static str,
        accountant: Arc<dyn Accountant>,
        sniffer: Arc<dyn Sniffer>,
        trader: Arc<dyn Trader>,
    ) -> Merchant {
        Merchant {
            id,
            accountant,
            sniffer,
            trader,
        }
    }
}

impl Default for Merchant {
    fn default() -> Self {
        Merchant {
            id: "Test",
            accountant: Arc::new(crate::accountant::Accountant::default()),
            trader: Arc::new(crate::trader::Trader::default()),
            sniffer: Arc::new(crate::sniffer::Sniffer::default()),
        }
    }
}

impl agnostic::merchant::Merchant for Merchant {
    fn id(&self) -> &'static str {
        self.id
    }

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
            0);
    }
}
