use std::{time::Duration, fmt::Display};

use rand::Rng;

#[derive(Debug, Clone)]
pub struct WeatherInfo {
    pub location: String,
    pub temperature: f64,
    pub humidity: f64,
    pub cloud_cover: CloudCover
}

#[derive(Debug, Clone)]
pub enum CloudCover {
    Clear, PartlyCloudy, Overcast
}

impl Display for CloudCover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloudCover::Clear => f.write_str("Clear"),
            CloudCover::PartlyCloudy => f.write_str("Partly Cloudy"),
            CloudCover::Overcast => f.write_str("Overcast"),
        }
    }
}

pub async fn coords_by_ip() -> Option<(f64, f64)> {
    let url = "https://ifconfig.co/";

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("Host", "ifconfig.co")
        .header("Accept", "application/json")
        .header("Connection", "close")
        .send()
        .await;
    if response.is_err() {
        return None;
    }

    let json = response.unwrap().json::<serde_json::Value>().await;
    if json.is_err() {
        return None;
    }

    let json = json.unwrap();
    let latitude = json["latitude"].as_f64().unwrap();
    let longitude = json["longitude"].as_f64().unwrap();

    Some((latitude, longitude))
}

pub async fn weather_for_coords(_lat: f64, _long: f64) -> Option<WeatherInfo> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    let mut rng = rand::thread_rng();
    let city = match rng.gen_range(0..=2) {
        0 => "Wellington".to_string(),
        1 => "Auckland".to_string(),
        _ => "Christchurch".to_string(),
    };
    
    let clouds = match rng.gen_range(0..=2) {
        0 => CloudCover::Clear,
        1 => CloudCover::PartlyCloudy,
        _ => CloudCover::Overcast,
    };

    Some(WeatherInfo {
        location: city,
        temperature: rng.gen_range(5.0..=25.),
        humidity: rng.gen_range(0.0..=100.),
        cloud_cover: clouds,
    })
}
