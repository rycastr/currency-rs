use std::error;

use async_trait::async_trait;

use crate::{repository::CurrencyRepository, Currency};

pub struct CurrencyRepoMemory {
    pub data: Vec<Currency>,
}

#[async_trait]
impl CurrencyRepository for CurrencyRepoMemory {
    async fn save(&mut self, currency: Currency) -> Result<(), Box<dyn error::Error>> {
        self.data.push(currency);
        Ok(())
    }

    async fn all(&self) -> Result<Vec<Currency>, Box<dyn error::Error>> {
        Ok(self.data.clone())
    }
}
