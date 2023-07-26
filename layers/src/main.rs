use element_wrapper::ElementWrapper;

use iced::{Sandbox, widget::{text, button, container}, Settings, Length, Rectangle, Point, Size, Element, Background, Color};
use iced::widget::column;
use layer::Layer;

use crate::layer_manager::LayerManager;

mod layer_manager;
mod layer;
mod element_wrapper;

pub fn border<T: 'static>(content: Element<T>) -> container::Container<T> {

    let style = |theme: &iced::Theme| -> container::Appearance {
        let palette = theme.extended_palette();
        let bg_color = Color::WHITE;
        container::Appearance {
            border_width: 2.,
            border_color: palette.primary.base.color,
            background: Some(Background::Color(bg_color)),
            ..Default::default()
        }
    } as for<'r> fn(&'r _) -> _;

    iced::widget::container(content)
        .padding(10)
        .style(style)
        .into()
}

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

        fn button_test(label: &str, on_pressed: &str) -> Element<'static, Message> {
            button(text(label.to_string()).size(30)).on_press(Message::SetMessage(on_pressed.to_string())).into()
        }

        fn rect(x: f32, y: f32, width: f32, height: f32) -> Rectangle<f32> {
            Rectangle::new(Point::new(x, y), Size::new(width, height))
        }

        column![
            ElementWrapper::new(button_test("wrapper", "wrapper pressed")),
            ElementWrapper::new(button_test("wrapper", "wrapper pressed")),
            ElementWrapper::new(button_test("wrapper", "wrapper pressed")),
            ElementWrapper::new(button_test("wrapper", "wrapper pressed")),
            Layer::new(
                rect(10., 10., 300., 300.), 
                button_test("simple layer", "simple layer pressed")),
            Layer::new(
                rect(20., 20., 300., 300.), 
                border(button_test("panel 1", "panel 1 pressed")).into()),
            LayerManager::new(vec![
                layer_manager::Layer::new(
                    rect(200., 200., 300., 300.), 
                    button_test("manager layer 1", "manager layer 1 pressed")
                ),
                layer_manager::Layer::new(
                    rect(220., 220., 300., 300.), 
                    button_test("manager layer 2", "manager layer 2 pressed")
                ),
                layer_manager::Layer::new(
                    rect(240., 240., 300., 300.), 
                    button_test("manager layer 3", "manager layer 3 pressed")
                ),
            ]),
            text(&self.message)
        ].width(Length::Fill).height(Length::Fill).into()
        
    }
}

fn main() -> Result<(), iced::Error> {
    LayersApp::run(Settings::default())
}
