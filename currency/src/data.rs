
pub const CURRENCIES: [&str; 5] = [
    "NZD",
    "AUS",
    "USD",
    "EUR",
    "JPY",
];

pub async fn calculate(value: f64, from_denom: String, to_denom: String) -> String {
    let url = format!("https://www.google.com/finance/quote/{from_denom}-{to_denom}");

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("Host", "www.google.com")
        .send()
        .await
        .unwrap();

    let body = response.text().await.unwrap();
    let token = "<div class=\"YMlKec fxKbKc\">";
    let index_1 = body.find(token).unwrap() + token.len();
    let index_2 = body[index_1..].find("</div>").unwrap();
    let rate = &body[index_1..(index_2 + index_1)];
    let rate = rate.parse::<f64>().unwrap();
    let value = rate * value;
    format!("{:.2}", value)
}