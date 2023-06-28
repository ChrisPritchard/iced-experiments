use iced::{Sandbox, widget::{column, text, container, row, button, Button, Text}, Settings, Length, alignment::{Horizontal, Vertical}};

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
            }
        }    
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {

        fn centred_text(label: &str) -> Text {
            text(label).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center)
        }

        fn pad_button(label: &str, msg: CalcMessage) -> Button<CalcMessage> {
            button(centred_text(label))
                .width(50)
                .height(60)
                .on_press(msg)
        }

        let content = 
            column![
                text("formula"),
                text(&self.displayed).size(20),
                row![
                    pad_button("7", CalcMessage::Number(7)),
                    pad_button("8", CalcMessage::Number(8)),
                    pad_button("9", CalcMessage::Number(9)),
                    pad_button("÷", CalcMessage::Op(CalcOp::Div)),
                ].spacing(10),
                row![
                    pad_button("4", CalcMessage::Number(4)),
                    pad_button("5", CalcMessage::Number(5)),
                    pad_button("6", CalcMessage::Number(6)),
                    pad_button("×", CalcMessage::Op(CalcOp::Mul)),
                ].spacing(10),
                row![
                    pad_button("1", CalcMessage::Number(1)),
                    pad_button("2", CalcMessage::Number(2)),
                    pad_button("3", CalcMessage::Number(3)),
                    pad_button("+", CalcMessage::Op(CalcOp::Add)),
                ].spacing(10),
                row![
                    pad_button("±", CalcMessage::Negate),
                    pad_button("0", CalcMessage::Number(0)),
                    pad_button(".", CalcMessage::Fraction),
                    pad_button("-", CalcMessage::Op(CalcOp::Sub)),
                ].spacing(10),
                button(centred_text("="))
                    .width(230)
                    .height(60)
                    .on_press(CalcMessage::Eval),
            ].spacing(10);

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
