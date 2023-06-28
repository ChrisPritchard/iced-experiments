use iced::{Sandbox, widget::{column, text, container, row, button, Button, Text}, Settings, Length, alignment::{Horizontal, Vertical}};

struct CalcState {
    // formula: String,
    displayed: String,
    left: Option<f64>,
    op: Option<CalcOp>,
    overwrite_display: bool,
    right: Option<f64>,
}

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

impl Sandbox for CalcState {
    type Message = CalcMessage;

    fn new() -> Self {
        Self {
            // formula: "".into(),
            displayed: "0".into(),
            left: None,
            op: None,
            overwrite_display: false,
            right: None,
        }
    }

    fn title(&self) -> String { "Iced Calc".into() }

    fn update(&mut self, message: Self::Message) {

        fn calc_result(a: f64, b: f64, op: &CalcOp) -> f64 {
            match op {
                CalcOp::Add => a + b,
                CalcOp::Sub => a - b,
                CalcOp::Mul => a * b,
                CalcOp::Div => a / b,
            }
        }

        match message {
            // displayed is used to track the current number being entered, before it is set as part of the formula, operation etc
            CalcMessage::Number(n) => {
                if self.displayed == "0" || self.overwrite_display {
                    self.displayed = format!("{n}");
                    self.overwrite_display = false;
                } else {
                    self.displayed = format!("{}{n}", self.displayed);
                }
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
            // op and eval finally parse numbers into positions, and possibly calculate results
            CalcMessage::Op(op) => { 
                let number: f64 = self.displayed.parse().unwrap();
                if self.left.is_none() {
                    self.left = Some(number);
                } else if self.right.is_some() {
                    let result = calc_result(self.left.unwrap(), self.right.unwrap(), &op); // calculate prior op
                    self.displayed = result.to_string();
                    self.left = Some(result);
                }
                self.op = Some(op);
                self.overwrite_display = true;
            },
            CalcMessage::Eval => {
                let number: f64 = self.displayed.parse().unwrap();
                if self.op.is_none() {
                    return;
                }
                if self.left.is_none() {
                    self.left = Some(number);
                    return;
                }
                if self.right.is_none() {
                    self.right = Some(number);
                }
                let result = calc_result(self.left.unwrap(), self.right.unwrap(), self.op.as_ref().unwrap());
                self.displayed = result.to_string();
                self.left = Some(result);
                self.overwrite_display = true;
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
