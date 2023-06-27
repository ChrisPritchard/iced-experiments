use iced::{Sandbox, widget::{column, text, container, row, button}, Settings, Length, alignment::{Horizontal, Vertical}};

#[derive(Debug, Clone)]
enum CalcMessage {
    Number(u32),
    Op(CalcOp),
    Negate, Fraction,
    Eval
}

#[derive(Debug, Clone)]
enum CalcOp {
    Add, Sub, Mul, Div
}

struct CalcState {
    displayed: String,
    current_op: Option<CalcOp>,
    op_just_selected: bool,
    last_entered: Option<f64>
}

impl Sandbox for CalcState {
    type Message = CalcMessage;

    fn new() -> Self {
        Self {
            displayed: "0".into(),
            current_op: None,
            op_just_selected: false,
            last_entered: None
        }
    }

    fn title(&self) -> String { "Iced Calc".into() }

    fn update(&mut self, message: Self::Message) {
        match message {
            CalcMessage::Op(op) => { 
                self.current_op = Some(op);
                self.op_just_selected = true;
            },
            CalcMessage::Number(n) => {
                if self.displayed == "0" || self.op_just_selected {
                    if self.op_just_selected {
                        self.last_entered = Some(self.displayed.parse().unwrap());
                    }
                    self.displayed = format!("{n}");
                } else {
                    self.displayed = format!("{}{n}", self.displayed);
                }
                self.op_just_selected = false;
            },
            CalcMessage::Negate => {
                if self.displayed.starts_with("-") {
                    self.displayed = self.displayed[1..].to_owned();
                } else {
                    self.displayed = format!("-{}", self.displayed);
                }
            },
            CalcMessage::Fraction => {
                if !self.displayed.contains(".") {
                    self.displayed = format!("{}.", self.displayed);
                }
            },
            CalcMessage::Eval => {
                if self.current_op.is_none() {
                    return;
                }
                let op = self.current_op.as_ref().unwrap();
                let number: f64 = self.displayed.parse().unwrap();
            }
        }    
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
                    button("÷").on_press(CalcMessage::Op(CalcOp::Div)),
                ],
                row![
                    button("4").on_press(CalcMessage::Number(4)),
                    button("5").on_press(CalcMessage::Number(5)),
                    button("6").on_press(CalcMessage::Number(6)),
                    button("×").on_press(CalcMessage::Op(CalcOp::Mul)),
                ],
                row![
                    button("1").on_press(CalcMessage::Number(1)),
                    button("2").on_press(CalcMessage::Number(2)),
                    button("3").on_press(CalcMessage::Number(3)),
                    button("+").on_press(CalcMessage::Op(CalcOp::Add)),
                ],
                row![
                    button("±").on_press(CalcMessage::Negate),
                    button("0").on_press(CalcMessage::Number(0)),
                    button(".").on_press(CalcMessage::Fraction),
                    button("-").on_press(CalcMessage::Op(CalcOp::Sub)),
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
