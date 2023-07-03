use iced::{Settings, Element, Length, Application, Command, Renderer, executor, theme};
use iced::widget::{row,column,text,text_input,button};

struct WeatherHere {
    latitude: Option<f32>, 
    longitude: Option<f32>,
    status: String,
    location: String,
    temperature: f32,
    weather: String,
}

#[derive(Debug, Clone)]
enum Message {
    FetchCoords,
    SetLat(String),
    SetLong(String),
}

impl Application for WeatherHere {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                latitude: None,
                longitude: None,
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

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let lat = if self.latitude.is_some() { 
            self.latitude.unwrap().to_string()
        } else { "".into() };
        let long = if self.longitude.is_some() { 
            self.longitude.unwrap().to_string() 
        } else { "".into() };

        column(vec![
            row(vec![
                text("Coords:").into(), 
                button("Guess from public IP").on_press(Message::FetchCoords).into()
                ]).into(),
            row(vec![
                text("Lat:").into(),
                text_input("Latitude", &lat).on_input(Message::SetLat).into(),
                text("Long:").into(),
                text_input("Longitude", &long).on_input(Message::SetLong).into(),
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
