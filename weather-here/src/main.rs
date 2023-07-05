
use data::WeatherInfo;
use iced::alignment::Vertical;
use iced::{Settings, Element, Length, Application, Command, Renderer, executor, Color};
use iced::widget::{row,column,text,text_input,button};

use crate::style::TextBoxValid;

mod data;
mod style;

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
                return;
            }
            let long = this.longitude.parse::<f64>();
            if long.is_err() || long.as_ref().unwrap().clamp(-180., 180.) != long.unwrap() {
                this.invalid_coord = true;
                return;
            }
            this.invalid_coord = false;
        }
        match message {
            Message::FetchCoords => {
                Command::perform(data::coords_by_ip(), Message::CoordReceived)
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
                Command::perform(data::weather_for_coords(lat, long), Message::WeatherReceived)
            },
            Message::WeatherReceived(w) => {
                self.weather = w;
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {

        fn center_left_text(t: &str) -> Element<Message> {
            text(t).height(Length::Fill).vertical_alignment(Vertical::Center).into()
        }

        let lat_style = TextBoxValid { 
            valid: self.latitude.parse::<f64>().is_ok(), 
            green: Color::from_rgb(0., 0.7, 0.), 
            red: Color::from_rgb(1., 0., 0.) 
        };

        let long_style = TextBoxValid { 
            valid: self.longitude.parse::<f64>().is_ok(), 
            ..lat_style
        };

        column(vec![
            row(vec![
                center_left_text("Coords:"),
                button("Guess from public IP").on_press(Message::FetchCoords).into()
                ]).height(30).spacing(20).into(),
            row(vec![
                center_left_text("Lat:"),
                text_input("Latitude", &self.latitude)
                    .style(lat_style)
                    .on_input(Message::SetLat).into(),
                center_left_text("Long:"),
                text_input("Longitude", &self.longitude)
                    .style(long_style)
                    .on_input(Message::SetLong).into(),
                ]).height(25).spacing(10).into(),
            button("Fetch Weather").on_press(Message::FetchWeather).into(),
            if self.weather.is_some() {
                let weather = self.weather.as_ref().unwrap();
                row(vec![
                    column(vec![
                        text(&weather.location).into(),
                        text(format!("{:.1} °C", &weather.temperature)).size(80).into()
                    ]).width(Length::FillPortion(2)).into(),
                    column(vec![
                        text(format!("{}", &weather.cloud_cover)).into(),
                        text(format!("{:.1}% humidity", &weather.humidity)).into()
                    ]).width(Length::FillPortion(1)).spacing(10).into(),
                ]).spacing(20).into()
            } else {
                text(if self.invalid_coord { "Set valid lat and long" } 
                    else { "No weather retrieved" }).into()
            }
        ])
            .padding(10)
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400, 250);
    settings.window.resizable = false;
    WeatherHere::run(settings)
}
