use std::time::Duration;

use iced::{Settings, Element, Length, Application, Command, Renderer, executor};
use iced::widget::{row,column,text,text_input,button};
use rand::Rng;

struct WeatherHere {
    latitude: String,
    longitude: String,
    invalid_coord: bool,
    weather: Option<WeatherInfo>,
}

#[derive(Debug, Clone)]
enum Message {
    FetchCoords,
    CoordReceived(Option<(f64, f64)>),
    SetLat(String),
    SetLong(String),
    FetchWeather,
    WeatherReceived(Option<WeatherInfo>),
}

async fn coords_by_ip() -> Option<(f64, f64)> {
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

#[derive(Debug, Clone)]
struct WeatherInfo {
    location: String,
    temperature: f64,
    humidity: f64,
    cloud_cover: CloudCover
}

#[derive(Debug, Clone)]
enum CloudCover {
    Clear, PartlyCloudy, Overcast
}

async fn weather_for_coords(_lat: f64, _long: f64) -> Option<WeatherInfo> {
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

impl Application for WeatherHere {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                latitude: "".into(),
                longitude: "".into(),
                invalid_coord: true,
                weather: None,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "WeatherHERE".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        fn check_invalid(this: &mut WeatherHere) {
            let lat = this.latitude.parse::<f64>();
            if lat.is_err() || lat.as_ref().unwrap().clamp(-90., 90.) != lat.unwrap() {
                this.invalid_coord = true;
            }
            let long = this.longitude.parse::<f64>();
            if long.is_err() || long.as_ref().unwrap().clamp(-180., 180.) != long.unwrap() {
                this.invalid_coord = true;
            }
            this.invalid_coord = false;
        }
        match message {
            Message::FetchCoords => {
                Command::perform(coords_by_ip(), Message::CoordReceived)
            },
            Message::SetLat(s) => {
                self.latitude = s;
                check_invalid(self);
                Command::none()
            },
            Message::SetLong(s) => {
                self.longitude = s;
                check_invalid(self);
                Command::none()
            }
            Message::CoordReceived(o) => {
                if o.is_none() {
                    self.invalid_coord = true;
                    return Command::none()
                }
                let (lat, long) = o.unwrap();
                self.latitude = lat.to_string();
                self.longitude = long.to_string();
                check_invalid(self);
                Command::none()
            },
            Message::FetchWeather => {
                if self.invalid_coord {
                    return Command::none()
                }
                let lat = self.latitude.parse::<f64>().unwrap();
                let long = self.longitude.parse::<f64>().unwrap();
                Command::perform(weather_for_coords(lat, long), Message::WeatherReceived)
            },
            Message::WeatherReceived(w) => {
                self.weather = w;
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {

        column(vec![
            row(vec![
                text("Coords:").into(),
                button("Guess from public IP").on_press(Message::FetchCoords).into()
                ]).into(),
            row(vec![
                text("Lat:").into(),
                text_input("Latitude", &self.latitude).on_input(Message::SetLat).into(),
                text("Long:").into(),
                text_input("Longitude", &self.longitude).on_input(Message::SetLong).into(),
                ]).into(),
            button("Fetch Weather").on_press(Message::FetchWeather).into()
        ])
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400, 300);
    settings.window.resizable = false;
    WeatherHere::run(settings)
}
