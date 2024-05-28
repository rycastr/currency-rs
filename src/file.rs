use std::error;

use async_trait::async_trait;

use crate::{repository::CurrencyRepository, Currency};

pub struct CurrencyRepoFile {
    pub file_path: String,
}

#[async_trait]
impl CurrencyRepository for CurrencyRepoFile {
    async fn save(&mut self, currency: Currency) -> Result<(), Box<dyn error::Error>> {
        let file_content = tokio::fs::read(&self.file_path).await?;
        let mut currencies: Vec<Currency> = serde_json::from_slice(&file_content)?;
        currencies.push(currency);

        let currency_data = serde_json::to_vec(&currencies)?;
        tokio::fs::write(&self.file_path, currency_data).await?;

        Ok(())
    }

    async fn all(&self) -> Result<Vec<Currency>, Box<dyn error::Error>> {
        let file_content = tokio::fs::read(&self.file_path).await?;

        let currencies: Vec<Currency> = serde_json::from_slice(&file_content)?;

        Ok(currencies)
    }
}
