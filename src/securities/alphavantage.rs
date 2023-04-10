use serde::Deserialize;
use std::{error::Error, fmt};

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

#[derive(Debug)]
pub struct RequestError {
    pub message: String,
}

impl Error for RequestError {}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request error")
    }
}

pub async fn get_security_price(security: &str) -> Result<String, RequestError> {
    let mut query = Query::default();
    query.function = String::from("GLOBAL_QUOTE");

    let request_url = query.request_url(security);

    let response_result = reqwest::get(&request_url).await;
    let response = match response_result {
        Ok(r) => r,
        Err(e) => {
            return Err(RequestError {
                message: e.to_string(),
            })
        }
    };

    let quote_returns_result = response.json::<AlphaVantageResponse>().await;
    let quote_returns = match quote_returns_result {
        Ok(q) => q,
        Err(e) => {
            return Err(RequestError {
                message: e.to_string(),
            })
        }
    };

    match quote_returns {
        AlphaVantageResponse::QuoteFound(r) => Ok(r.global_quote.price),
        AlphaVantageResponse::RequestLimitExceeded(n) => Err(RequestError { message: n.note }),
        _ => Err(RequestError {
            message: "The security was not found".to_string(),
        }),
    }
}
