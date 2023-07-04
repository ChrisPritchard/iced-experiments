use iced::theme::TextInput;
use iced::{Settings, Element, Length, Application, Command, Renderer, executor, theme, Theme, Color};
use iced::widget::{row,column,text,text_input,button};
use serde_json::Value;

struct WeatherHere {
    latitude: String, 
    longitude: String,
    status: String,
    location: String,
    temperature: f64,
    weather: String,
}

#[derive(Debug, Clone)]
enum Message {
    FetchCoords,
    CoordReceived(Option<(f64, f64)>),
    SetLat(String),
    SetLong(String),
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
                status: "".into(),
                location: "".into(),
                temperature: 0.,
                weather: "".into()
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "WeatherHERE".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::FetchCoords => { 
                Command::perform(coords_by_ip(), Message::CoordReceived) 
            },
            Message::SetLat(s) => {
                self.latitude = s;
                Command::none()
            },
            Message::SetLong(s) => {
                self.longitude = s;
                Command::none()
            }
            Message::CoordReceived(o) => {
                if o.is_none() {
                    return Command::none()
                }
                let (lat, long) = o.unwrap();
                self.latitude = lat.to_string();
                self.longitude = long.to_string();
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {

        let valid_style: for<'r> fn(&'r _) -> _ = |_: &Theme| -> text_input::Appearance {
            text_input::Appearance { 
                border_color: Color::from_rgb(0., 1., 0.),
                background: Color::TRANSPARENT.into(),
                border_radius: 0.,
                border_width: 1.,
                icon_color: Color::BLACK,
            }
        };
        let valid_style = Box::new(valid_style);

        let invalid_style: for<'r> fn(&'r _) -> _ = |_: &Theme| -> text_input::Appearance {
            text_input::Appearance { 
                border_color: Color::from_rgb(1., 0., 0.),
                background: Color::TRANSPARENT.into(),
                border_radius: 0.,
                border_width: 1.,
                icon_color: Color::BLACK,
            }
        };

        column(vec![
            row(vec![
                text("Coords:").into(), 
                button("Guess from public IP").on_press(Message::FetchCoords).into()
                ]).into(),
            row(vec![
                text("Lat:").into(),
                text_input("Latitude", &self.latitude)
                    .style(TextInput::Custom(valid_style))
                    .on_input(Message::SetLat)
                    .into(),
                text("Long:").into(),
                text_input("Longitude", &self.longitude).on_input(Message::SetLong).into(),
                ]).into(),
            text(&self.status).into()
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
