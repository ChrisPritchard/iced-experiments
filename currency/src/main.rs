use iced::{Application, executor, Command, widget::{pick_list, text_input, row, column}, Length};

mod data;

struct ValidValue {
    value: String,
}

impl ValidValue {
    fn valid_amount(&self) -> Option<f64> {
        self.value.parse::<f64>().ok()
    }
}

struct Currency {
    currencies: Vec<String>,
    from_denom: String,
    to_denom: String,
    from: ValidValue,
    to: ValidValue,
}

#[derive(Debug, Clone)]
enum Message {
    FromDenom(String),
    ToDenom(String),
    EnterFrom(String),
    EnterTo(String),
    FromToConversion(String),
    ToFromConversion(String),
}

impl Application for Currency {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                currencies: data::CURRENCIES.iter().map(|s| s.to_string()).collect(),
                from_denom: "NZD".into(),
                to_denom: "NZD".into(),
                from: ValidValue { value: "0.00".into() },
                to: ValidValue { value: "0.00".into() },
            }, Command::none()
        )
    }

    fn title(&self) -> String {
        "Currency Converter".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::EnterFrom(v) => {
                self.from.value = v;
                if let Some(c) = self.from.valid_amount() {
                    Command::perform(data::calculate(c, self.from_denom.clone(), self.to_denom.clone()), Message::FromToConversion)
                } else {
                    Command::none()
                }
            },
            Message::EnterTo(v) => {
                self.to.value = v;
                if let Some(c) = self.to.valid_amount() {
                    Command::perform(data::calculate(c, self.to_denom.clone(), self.from_denom.clone()), Message::ToFromConversion)
                } else {
                    Command::none()
                }
            },
            Message::FromDenom(c) => {
                self.from_denom = c;
                if let Some(c) = self.from.valid_amount() {
                    Command::perform(data::calculate(c, self.from_denom.clone(), self.to_denom.clone()), Message::FromToConversion)
                } else {
                    Command::none()
                }
            },
            Message::ToDenom(c) => {
                self.to_denom = c;
                if let Some(c) = self.to.valid_amount() {
                    Command::perform(data::calculate(c, self.to_denom.clone(), self.from_denom.clone()), Message::ToFromConversion)
                } else {
                    Command::none()
                }
            },
            Message::FromToConversion(s) => {
                self.to.value = s;
                Command::none()
            },
            Message::ToFromConversion(s) => {
                self.from.value = s;
                Command::none()
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        row(vec![
            column(vec![
                pick_list(&self.currencies, Some(self.from_denom.clone()), Message::FromDenom).into(),
                text_input("Enter Value", &self.from.value)
                    .size(70)
                    .on_input(Message::EnterFrom).into(),
            ]).spacing(10).width(Length::FillPortion(1)).into(),
            column(vec![
                pick_list(&self.currencies, Some(self.to_denom.clone()), Message::ToDenom).into(),
                text_input("Enter Value", &self.to.value)
                    .size(70)
                    .on_input(Message::EnterTo).into(),
            ]).spacing(10).width(Length::FillPortion(1)).into(),
        ]).spacing(10).padding(10).into()
    }
}

fn main() -> iced::Result {
    let settings = iced::Settings {
        window: iced::window::Settings {
            size: (400, 150),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    };
    Currency::run(settings)
}
