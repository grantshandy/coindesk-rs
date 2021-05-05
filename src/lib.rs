//! # coindesk
//! A Bitcoin price index API powered by [coindesk.com](https://coindesk.com).
//!
//! ```
//! coindesk = "0.1.0"
//! ```
//!
//! Because this is an async library you will need an async runtime like `async_std` or `tokio`.
//!
//! Example:
//! ```rust
//! use coindesk::Bitcoin;
//!
//! #[tokio::main]
//! async fn main() {
//!     let data = Bitcoin::get().await.unwrap();
//!
//!     println!("currency: {}, rate: {}, description: {}, symbol: {}", data.usd.code, data.usd.rate, data.usd.description, data.usd.symbol);
//!     println!("currency: {}, rate: {}, description: {}, symbol: {}", data.gbp.code, data.gbp.rate, data.gbp.description, data.gbp.symbol);
//!     println!("currency: {}, rate: {}, description: {}, symbol: {}", data.eur.code, data.eur.rate, data.eur.description, data.eur.symbol);
//!     println!("time: {}", data.time);
//! }
//! ```
//!
//! Output:
//! ```
//! currency: USD, rate: 57532.8599, description: United States Dollar, symbol: &#36;
//! currency: GBP, rate: 41359.4525, description: British Pound Sterling, symbol: &pound;
//! currency: EUR, rate: 47927.864, description: Euro, symbol: &euro;
//! time: 2021-05-05 15:58:00 UTC
//! ```
//! The time field implements `chrono::DateTime` for integration with the rest of your project.
//!
//! Disclaimer: This data was produced from the CoinDesk Bitcoin Price Index (USD). Non-USD currency data converted using hourly conversion rate from [openexchangerates.org](https://openexchangerates.org)
//!
//! Made with ❤️ in 🦀 by Grant Handy.


use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Debug)]
pub struct Currency {
    pub code: String,
    pub symbol: String,
    pub description: String,
    pub rate: f64,
}

impl Currency {
    pub fn get(ctype: &str, data: Value) -> Result<Self, Error> {
        let code = match &data["bpi"][ctype]["code"] {
            Value::String(output) => output,
            _ => return Err(Error::Format(String::from("Couldn't find code"))),
        };

        let symbol = match &data["bpi"][ctype]["symbol"] {
            Value::String(output) => output,
            _ => return Err(Error::Format(String::from("Couldn't find symbol"))),
        };

        let description = match &data["bpi"][ctype]["description"] {
            Value::String(output) => output,
            _ => return Err(Error::Format(String::from("Couldn't find description"))),
        };

        let rate = match &data["bpi"][ctype]["rate_float"] {
            Value::Number(output) => match output.as_f64() {
                Some(num) => num,
                None => return Err(Error::Format(String::from("Rate is not f64"))),
            },
            _ => return Err(Error::Format(String::from("Couldn't find rate"))),
        };

        Ok(Self {
            code: code.to_owned(),
            symbol: symbol.to_owned(),
            description: description.to_owned(),
            rate,
        })
    }
}

#[derive(Clone, PartialEq, Hash, Debug)]
pub enum Error {
    Http(String),
    Format(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Http(error) => {
                write!(f, "HTTP Error: {}", error)
            }
            Error::Format(error) => {
                write!(f, "Formatting Error {}", error)
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Bitcoin {
    pub time: DateTime<Utc>,
    pub usd: Currency,
    pub gbp: Currency,
    pub eur: Currency,
}

impl Bitcoin {
    pub async fn get() -> Result<Self, Error> {
        let data = match surf::get("https://api.coindesk.com/v1/bpi/currentprice.json").recv_string().await {
            Ok(data) => data,
            Err(error) => return Err(Error::Http(error.to_string())),
        };

        let parsed_json: Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(Error::Format(error.to_string()));
            }
        };

        let usd = Currency::get("USD", parsed_json.clone())?;
        let gbp = Currency::get("GBP", parsed_json.clone())?;
        let eur = Currency::get("EUR", parsed_json.clone())?;

        let time = match &parsed_json["time"]["updatedISO"] {
            Value::String(output) => output,
            _ => return Err(Error::Format(String::from("Couldn't find updatedISO"))),
        };

        let time: DateTime<Utc> = match DateTime::parse_from_rfc3339(time) {
            Ok(time) => DateTime::from(time),
            Err(error) => return Err(Error::Format(format!("Couldn't format time: {}", error))),
        };

        Ok(Self {
            time,
            usd,
            gbp,
            eur,
        })
    }
}
