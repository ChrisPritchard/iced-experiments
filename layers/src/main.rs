use iced::{Sandbox, widget::text, Settings};


struct LayersApp {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for LayersApp {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        "Layers App".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        text("hello world").into()
    }
}

fn main() -> Result<(), iced::Error> {
    LayersApp::run(Settings::default())
}
