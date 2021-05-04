
use serde_json::Value;

#[derive(Clone, PartialEq, Debug)]
pub struct Currency {
    pub code: String,
    pub symbol: String,
    pub description: String,
    pub rate: f64,
}

impl Currency {
    pub fn get(data: serde_json::Value) -> Result<Self, Error> {
        let code = match &data["code"] {
            Value::String(output) => output,
            _ => return Err(Error::Format(String::from("Couldn't find code"))),
        };

        let symbol = match &data["symbol"] {
            Value::String(output) => output,
            _ => return Err(Error::Format(String::from("Couldn't find symbol"))),
        };

        let description = match &data["description"] {
            Value::String(output) => output,
            _ => return Err(Error::Format(String::from("Couldn't find description"))),
        };

        let rate = match &data["rate"] {
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
    pub usd: Currency,
    pub gbp: Currency,
    pub eur: Currency,
}

impl Bitcoin {
    pub async fn get() -> Result<String, Error> {
        let data = match surf::get("https://api.coindesk.com/v1/bpi/currentprice.json").recv_string().await {
            Ok(data) => data,
            Err(error) => return Err(Error::Http(error.to_string())),
        };

        let parsed_json: serde_json::Value = match serde_json::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(error) => {
                return Err(Error::Format(error.to_string()));
            }
        };

        let usd_raw_data = match &parsed_json["usd"] {
            Value::Object(output) => output,
            _ => return Err(Error::Format(String::from("Couldn't find usd"))),
        };

        return Ok(usd_raw_data.to_string().to_owned());

        // let usd = Currency::get()

        // println!("{}", data);

        // Ok(Self {
        //     usd,
        //     gbp,
        //     eur,
        // })
    }
}