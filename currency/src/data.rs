use std::time::Duration;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;
use rand::Rng;

pub const CURRENCIES: [&str; 5] = [
    "NZD",
    "AUS",
    "USD",
    "EUR",
    "JPY",
];

pub async fn calculate(value: f64, from_denom: String, to_denom: String) -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;

    if from_denom == to_denom {
        return format!("{:.2}", value);
    }

    let d1 = CURRENCIES.iter().position(|f| **f == from_denom).unwrap();
    let d2 = CURRENCIES.iter().position(|f| **f == to_denom).unwrap();
    let seed = d1 * 10 + d2;

    let mut rng = ChaCha8Rng::seed_from_u64(seed as u64);
    let rate = rng.gen_range(0.1..2.);
    format!("{:.2}", value * rate)
}