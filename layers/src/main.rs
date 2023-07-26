use element_wrapper::ElementWrapper;

use iced::{Sandbox, widget::{text, button}, Settings, Length, Rectangle, Point, Size};
use iced::widget::column;
use layer::Layer;

mod layer;
mod element_wrapper;

struct LayersApp {
    message: String,
}

#[derive(Debug, Clone)]
enum Message {
    SetMessage(String)
}

impl Sandbox for LayersApp {
    type Message = Message;

    fn new() -> Self {
        Self { message: "just started".to_string() }
    }

    fn title(&self) -> String {
        "Layers App".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SetMessage(s) => self.message = s,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {

        let wrapper_content = button(text("wrapper")).on_press(Message::SetMessage("wrapper pressed".to_string()));
        let layer_content = button(text("layer")).on_press(Message::SetMessage("layer pressed".to_string())).into();

        column![
            ElementWrapper::new(wrapper_content.into()),
            Layer::new(
                Rectangle::new(Point::new(100., 100.), Size::new(300., 300.)), 
                layer_content),
            text(&self.message)
        ].width(Length::Fill).height(Length::Fill).into()
        
    }
}

fn main() -> Result<(), iced::Error> {
    LayersApp::run(Settings::default())
}
