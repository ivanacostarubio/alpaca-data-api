use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Config {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Clone, Debug)]
pub struct Stock {
    pub symbol: String,
    pub config: Config,
}

#[derive(Deserialize, Debug)]
pub struct StockLastTradeResponse {
    status: String,
    symbol: String,
    last: LastTrade,
}

#[derive(Deserialize, Debug)]
pub struct LastTrade {
    price: f64,
    size: f64,
    exchange: u16,
    cond1: u64,
    cond2: u64,
    cond3: u64,
    cond4: u64,
    timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct StockLastQuoteResponse {
    status: String,
    symbol: String,
    last: LastQuote,
}

#[derive(Deserialize, Debug)]
pub struct LastQuote {
    askprice: f64,
    asksize: u16,
    askexchange: u16,
    bidprice: f64,
    bidsize: u16,
    bidexchange: u16,
    timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct Bar {
    t: u64, // timestamp
    o: f64, // open
    h: f64, // high
    l: f64, // low
    c: f64, // close
    v: u64, // volume
}

pub type BarResponse = HashMap<String, Vec<Bar>>;

#[allow(dead_code)]
enum Duration {
    Minute,
    Min1,
    Min5,
    Min15,
    Day,
}

/*
* TODO:
  - ADD DURATION PARAMETERS
  - ADD QUERY PARAMETERS
  - Refactor 3 API calls into abstract. Passing URL, Parameters, Deserializer
*/

impl Stock {
    pub async fn bars(self) -> Result<BarResponse, Box<dyn std::error::Error>> {
        let url = format!("{:}", "https://data.alpaca.markets/v1/bars/1D");

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .header("APCA-API-KEY-ID", self.config.api_key)
            .header("APCA-API-SECRET-KEY", self.config.api_secret)
            .query(&[("symbols", self.symbol)])
            .send()
            .await?
            .json::<BarResponse>()
            .await?;

        for item in res.iter() {
            println!("{:?}", item);
        }

        Ok(res)
    }

    pub async fn last_trade(self) -> Result<StockLastTradeResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{:}/{:}",
            "https://data.alpaca.markets/v1/last/stocks", self.symbol
        );

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .header("APCA-API-KEY-ID", self.config.api_key)
            .header("APCA-API-SECRET-KEY", self.config.api_secret)
            .send()
            .await?
            .json::<StockLastTradeResponse>()
            .await?;

        Ok(res)
    }

    pub async fn last_quote(self) -> Result<StockLastQuoteResponse, Box<dyn std::error::Error>> {
        let url = format!(
            "{:}/{:}",
            "https://data.alpaca.markets/v1/last_quote/stocks", self.symbol
        );

        let client = reqwest::Client::new();
        let res = client
            .get(&url)
            .header("APCA-API-KEY-ID", self.config.api_key)
            .header("APCA-API-SECRET-KEY", self.config.api_secret)
            .send()
            .await?
            .json::<StockLastQuoteResponse>()
            .await?;

        Ok(res)
    }
}
