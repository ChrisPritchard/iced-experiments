use iced::{Sandbox, widget::{column, text, container, row, button}, Settings, Length, alignment::{Horizontal, Vertical}};

#[derive(Debug, Clone)]
enum CalcMessage {
    Number(u32),
    OpMul, OpDiv, OpAdd, OpSub,
    Negate, Fraction,
    Eval
}

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
            column![
                text("formula"),
                text("displayed"),
                row![
                    button("7").on_press(CalcMessage::Number(7)),
                    button("8").on_press(CalcMessage::Number(8)),
                    button("9").on_press(CalcMessage::Number(9)),
                    button("÷").on_press(CalcMessage::OpDiv),
                ],
                row![
                    button("4").on_press(CalcMessage::Number(4)),
                    button("5").on_press(CalcMessage::Number(5)),
                    button("6").on_press(CalcMessage::Number(6)),
                    button("×").on_press(CalcMessage::OpMul),
                ],
                row![
                    button("1").on_press(CalcMessage::Number(1)),
                    button("2").on_press(CalcMessage::Number(2)),
                    button("3").on_press(CalcMessage::Number(3)),
                    button("+").on_press(CalcMessage::OpAdd),
                ],
                row![
                    button("±").on_press(CalcMessage::Negate),
                    button("0").on_press(CalcMessage::Number(0)),
                    button(".").on_press(CalcMessage::Fraction),
                    button("-").on_press(CalcMessage::OpSub),
                ],
                button("=").on_press(CalcMessage::Eval),
            ];

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
