use iced::{Application, executor, Command, widget::text};


struct Currency {
    currencies: Vec<String>,
    from_denom: String,
    to_denom: String,
    from: f64,
    to: f64,
}

#[derive(Debug, Clone)]
enum Message {
    GetCurrencies,
    CurrenciesReceived(Vec<String>),
    EnterFrom(String),
    EnterTo(String),
}

impl Application for Currency {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {
            currencies: Vec::new(),
            from_denom: "".into(),
            to_denom: "".into(),
            from: 0.,
            to: 0.,
        }, Command::none())
    }

    fn title(&self) -> String {
        "Currency Converter".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        text("Currency Converter").into()
    }
}

fn main() -> iced::Result {
    let settings = iced::Settings {
        window: iced::window::Settings {
            size: (400, 300),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    };
    Currency::run(settings)
}
