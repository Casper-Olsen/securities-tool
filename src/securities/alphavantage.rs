use serde::Deserialize;
use thiserror::Error;

struct Query {
    function: String,
    api_key: String,
}

impl Query {
    fn request_url(&self, symbol: &str) -> String {
        format!(
            "https://www.alphavantage.co/query?function={function}&symbol={symbol}&apikey={api_key}",
            function = self.function,
            symbol = symbol,
            api_key = self.api_key
        )
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            function: Default::default(),
            api_key: "2J0BUQ8SZYKQ5OWW".to_string(),
        }
    }
}

#[derive(Deserialize)]
struct RootQuote {
    #[serde(rename = "Global Quote")]
    global_quote: GlobalQuote,
}

#[derive(Deserialize)]
struct GlobalQuote {
    #[serde(rename = "05. price")]
    price: String,
}

#[derive(Deserialize)]
struct Note {
    #[serde(rename = "Note")]
    note: String,
}

#[derive(Deserialize)]
struct EmptyQuote {}

#[derive(Deserialize)]
#[serde(untagged)] // https://serde.rs/enum-representations.html#untagged
enum AlphaVantageResponse {
    QuoteFound(RootQuote),
    RequestLimitExceeded(Note),
    QuoteNotFound(EmptyQuote),
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("The request failed")]
    RequestFailed(#[from] reqwest::Error),
    #[error("{0}")]
    LimitExceeded(String),
    #[error("The security was not found")]
    NotFound,
}

pub async fn get_security_price(security: &str) -> Result<String, RequestError> {
    let query = Query {
        function: String::from("GLOBAL_QUOTE"),
        ..Default::default() // Override 'function', but still retain the other defaults
    };

    let request_url = query.request_url(security);

    let response = reqwest::get(&request_url).await?; // If Err(e): convert to RequestError and return

    let quote_returns = response.json::<AlphaVantageResponse>().await?;

    match quote_returns {
        AlphaVantageResponse::QuoteFound(r) => Ok(r.global_quote.price),
        AlphaVantageResponse::RequestLimitExceeded(n) => Err(RequestError::LimitExceeded(n.note)),
        _ => Err(RequestError::NotFound),
    }
}
