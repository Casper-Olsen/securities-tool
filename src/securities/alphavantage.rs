use serde::Deserialize;

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
struct Root {
    #[serde(rename = "Global Quote")]
    global_quote: GlobalQuote,
}

#[derive(Deserialize)]
struct GlobalQuote {
    #[serde(rename = "05. price")]
    price: String,
}

pub async fn get_security_price(security: &str) -> Result<String, reqwest::Error> {
    let mut query = Query::default();
    query.function = String::from("GLOBAL_QUOTE");

    let request_url = query.request_url(security);

    let response = reqwest::get(&request_url).await?;

    let global_quote: Root = response.json().await?;

    return Ok(global_quote.global_quote.price);
}
