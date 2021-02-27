pub struct Trader {
}

impl agnostic::market::Trader for Trader {
    fn create_order(
        &self,
        _order: agnostic::order::Order
    ) -> agnostic::market::Future<Result<(), String>> {
        Box::pin(async {
            Ok(())
        })
    }

    fn delete_and_create(
        &self,
        _id: &str,
        _new_order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<String, String>> {
        Box::pin(async {
            Ok("Test".to_owned())
        })
    }

    fn delete_order(&self, _id: &str) -> agnostic::market::Future<Result<(), String>> {
        Box::pin(async {
            Ok(())
        })
    }

    fn create_trade_from_order(
        &self,
        _order: agnostic::order::Order
    ) -> agnostic::market::Future<Result<(), String>> {
        Box::pin(async {
            Ok(())
        })
    }
}
