use std::time::Duration;
use rand::Rng;

pub const CURRENCIES: [&str; 5] = [
    "NZD",
    "AUS",
    "USD",
    "EUR",
    "JPY",
];

pub async fn calculate(value: f64, _from_denom: String, _to_denom: String) -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    let mut rng = rand::thread_rng();
    let rate = rng.gen_range(0.1..2.);
    format!("{:.2}", value * rate)
}