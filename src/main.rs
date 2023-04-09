pub mod securities;

use colored::Colorize;
use std::env;

use crate::securities::commandline::commandline::get_security;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = env::args().collect::<Vec<String>>();
    let security = get_security(&args);

    println!("Finding price for security: {}", security.bold());

    let price = securities::alphavantage::get_security_price(&security).await?;

    println!("Price for {} is: {}", security.bold(), price.green());

    Ok(())
}
