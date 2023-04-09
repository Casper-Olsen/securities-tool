pub mod securities;

use std::env;

use crate::securities::commandline::commandline::get_security;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = env::args().collect::<Vec<String>>();
    let security = get_security(&args);

    println!("Finding price for security: {}", security);

    let price = securities::alphavantage::get_security_price(&security).await?;

    println!("Price for the security {} is: {}", security, price);

    Ok(())
}
