pub struct Trader {
}

impl agnostic::market::Trader for Trader {
    fn create_order(
        &self,
        order: agnostic::order::Order
    ) -> agnostic::market::Future<Result<(), String>> {
        Box::pin(async move {
            log::debug!("Creating order {:#?}", order);
            Ok(())
        })
    }

    fn delete_and_create(
        &self,
        id: &str,
        new_order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<String, String>> {
        let id = id.to_owned();
        Box::pin(async move {
            log::debug!("Deleting and creating order {:#?} with id {}", new_order, id);
            Ok("Test".to_owned())
        })
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        let id = id.to_owned();
        Box::pin(async move {
            log::debug!("Deleting order with id: {:#?}", id);
            Ok(())
        })
    }

    fn create_trade_from_order(
        &self,
        order: agnostic::order::Order
    ) -> agnostic::market::Future<Result<(), String>> {
        Box::pin(async move {
            log::debug!("Creating trade: {:#?}", order);
            Ok(())
        })
    }
}
