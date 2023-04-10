pub mod securities;

use colored::Colorize;
use std::env;

use crate::securities::commandline::get_security;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();

    let security_result = get_security(&args);
    let security = match security_result {
        Ok(s) => s,
        Err(error) => {
            println!("Args error: {}", error);
            return;
        }
    };

    println!("Finding price for security: {}", security.bold());

    let price_result = securities::alphavantage::get_security_price(&security).await;
    let price = match price_result {
        Ok(p) => p,
        Err(error) => {
            println!("Security not found: {}", error);
            return;
        }
    };

    println!("Price for {} is: {}", security.bold(), price.green());
}
