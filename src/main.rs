use iced::{Sandbox, widget::{column, text, container}, Settings, Length, alignment::{Horizontal, Vertical}};

#[derive(Debug)]
enum CalcMessage {}

enum CalcOp {
    Add, Sub, Mul, Div
}

struct CalcState {
    displayed: f64,
    current_op: Option<CalcOp>,
    last_entered: Option<f64>
}

impl Sandbox for CalcState {
    type Message = CalcMessage;

    fn new() -> Self {
        Self {
            displayed: 0.,
            current_op: None,
            last_entered: None
        }
    }

    fn title(&self) -> String { "Iced Calc".into() }

    fn update(&mut self, _message: Self::Message) {
        
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let content = 
            column!(
                text("formula"),
                text("displayed"),
                text("row 1"),
                text("row 2"),
                text("row 3"),
                text("row 4"),
                text("equals"),
            );
            
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center).into()
    }
}


fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (300, 400);
    CalcState::run(settings)
}
