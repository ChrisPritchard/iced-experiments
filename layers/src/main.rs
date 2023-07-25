use iced::{Sandbox, widget::text, Settings};
use iced_native::{layout::Node, Size, Layout, column};


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
        let n = Node::new(Size::new(300., 300.));
        
        let lt = Layout::new(&n);

        column![
            text("hello world"),
            n,
        ].into()
    }
}

fn main() -> Result<(), iced::Error> {
    LayersApp::run(Settings::default())
}
