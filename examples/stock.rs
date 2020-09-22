extern crate dotenv;

use dotenv::dotenv;

#[macro_use]
extern crate dotenv_codegen;

use alpaca_data_api::stock::{Config, Stock};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let portfolio = ["SHOP"];

    let config = Config {
        api_key: dotenv!("API_KEY").to_string(),
        api_secret: dotenv!("API_SECRET").to_string(),
    };

    for entry in portfolio.iter() {
        let stock = Stock {
            symbol: entry.to_string(),
            config: config.clone(),
        };

        let b = stock.clone().bars().await?;
        println!("{:#?}", b);

        let q = stock.clone().last_quote().await?;
        println!("{:#?}", q);

        let t = stock.clone().last_trade().await?;
        println!("{:#?}", t);
    }

    return Ok(());
}
