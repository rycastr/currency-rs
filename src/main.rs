use memory::CurrencyRepoMemory;
use repository::CurrencyRepository;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled};

pub mod file;
pub mod memory;
pub mod repository;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct CurrencyPrice {
    #[serde(rename = "paridadeCompra")]
    purchase_parity: f64,
    #[serde(rename = "paridadeVenda")]
    sale_parity: f64,
    #[serde(rename = "cotacaoCompra")]
    purchase_quote: f64,
    #[serde(rename = "cotacaoVenda")]
    sales_quote: f64,
    #[serde(rename = "dataHoraCotacao")]
    quote_datetime: String,
    #[serde(rename = "tipoBoletim")]
    bulletin_type: String,
}

#[derive(Debug, Deserialize)]
pub struct PriceResponse {
    value: Vec<CurrencyPrice>,
}

#[derive(Debug, Deserialize, Tabled, Clone, Serialize)]
#[allow(dead_code)]
pub struct Currency {
    #[serde(rename = "simbolo")]
    symbol: String,
    #[serde(rename = "nomeFormatado")]
    normalized_name: String,
    #[serde(rename = "tipoMoeda")]
    currency_type: String,
    #[tabled(display_with = "display_price")]
    price: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct CurrencyResponse {
    value: Vec<Currency>,
}

fn display_price(price_option: &Option<f64>) -> String {
    match price_option {
        Some(price) => format!("{price}"),
        None => "".to_string(),
    }
}

async fn save_currency(repo: &mut Box<dyn CurrencyRepository>, currency: Currency) {
    repo.save(currency).await.unwrap()
}

async fn fetch_price(currency: String) -> Result<CurrencyPrice, anyhow::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://olinda.bcb.gov.br/olinda/service/PTAX/version/v1/odata/ExchangeRateDate(moeda=@moeda,dataCotacao=@dataCotacao)")
        .query(&[("@dataCotacao", "'05-24-2024'"), ("@moeda", format!("'{currency}'").as_ref())])
        .send().await?;

    match response.status() {
        StatusCode::OK => {
            let price_response = response.json::<PriceResponse>().await?;
            let currency_price = price_response
                .value
                .first()
                .ok_or(anyhow::Error::msg("Fail on retrive currency price"))?;

            Ok(currency_price.clone())
        }
        status => Err(anyhow::Error::msg(format!(
            "Error on retrive currency price :: status({status})"
        ))),
    }
}

async fn fetch_currencies(
    currency_repository: &mut Box<dyn CurrencyRepository>,
) -> Result<(), anyhow::Error> {
    let response =
        reqwest::get("https://olinda.bcb.gov.br/olinda/service/PTAX/version/v1/odata/Currencies")
            .await
            .unwrap();

    match response.status() {
        StatusCode::OK => {
            let currency_response = response.json::<CurrencyResponse>().await.unwrap();
            for mut currency in currency_response.value {
                let currency_price = fetch_price(currency.symbol.clone()).await.unwrap();
                currency.price = Some(currency_price.sales_quote);
                save_currency(currency_repository, currency).await;
            }
        }
        status => println!("Error on request :: status {status}"),
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let mut currency_repository: Box<dyn CurrencyRepository> =
        Box::new(CurrencyRepoMemory { data: vec![] });

    fetch_currencies(&mut currency_repository).await.unwrap();

    let table = Table::new(currency_repository.all().await.unwrap()).to_string();
    println!("{table}");
}
