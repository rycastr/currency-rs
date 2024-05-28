use std::error;

use async_trait::async_trait;

use crate::Currency;

#[async_trait]
pub trait CurrencyRepository {
    async fn save(&mut self, currency: Currency) -> Result<(), Box<dyn error::Error>>;
    async fn all(&self) -> Result<Vec<Currency>, Box<dyn error::Error>>;
}
