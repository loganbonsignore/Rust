use actix_web::{get, web, App, HttpServer, Responder};
use reqwest::blocking::Client;

extern crate reqwest;

async fn coinbase(query_tup: &(&str, &str), client: Client) {
    // Calls coinbase API for price info (in developement)
    let query_str = format!("https://api.pro.coinbase.com/products/{ticker}-{currency}/ticker", ticker=query_tup.0, currency=query_tup.1);
    let response = reqwest::get(&query_str).await;
    format!("{:?}", response);
}

fn split_price_currency(pair: &str) -> (&str, &str) {
    // Splits individual query pair (Ex: btc_usd -> (btc, usd))
    let query_pair: Vec<&str> = pair.split("_").collect();
    (query_pair[0], query_pair[1])
}

#[get("/{pairs}")]
async fn aggregate_pairs(web::Path(pairs): web::Path<String>) -> impl Responder {
    // Split query string into individual pairs
    let split_pairs: Vec<&str> = pairs.split(",").collect();
    // Create client for repeated HTTP requests
    let client = Client::new();
    // Query each individual pair
    for pair in &split_pairs {
        let query_tup = split_price_currency(pair);
    }
    // Create JSON of data
    // Return JSON
    format!("Pairs: {:?}", split_pairs)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(aggregate_pairs))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}