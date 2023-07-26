
use iced::{Sandbox, widget::{text, button, container, text_input}, Settings, Length, Rectangle, Point, Size, Element, Background, Color};
use iced::widget::column;

use crate::overlay_manager::*;

mod overlay_manager;

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

        fn rect(x: f32, y: f32, width: f32, height: f32) -> Rectangle<f32> {
            Rectangle::new(Point::new(x, y), Size::new(width, height))
        }

        column![
            OverlayManager::new(vec![
                overlay_manager::Layer::new(
                    rect(200., 200., 300., 300.), 
                    border(column![
                        text("some sample text"),
                        text("on more than one line"),
                        text("to see how it looks"),
                    ].into()).into()
                ),
                overlay_manager::Layer::new(
                    rect(220., 220., 300., 300.), 
                    border(column![
                        text("some text and inputs"),
                        text_input("placeholder", &self.message),
                        button("button").on_press(Message::SetMessage("layer 2 pressed".into()))
                    ].into()).into()
                ),
                overlay_manager::Layer::new(
                    rect(240., 240., 300., 300.), 
                    border(column![
                        text("final layer with"),
                        text("yet another button"),
                        button("button").on_press(Message::SetMessage("layer 3 pressed".into()))
                    ].into()).into()
                ),
            ]),
            text(&self.message)
        ].width(Length::Fill).height(Length::Fill).into()
        
    }
}

fn main() -> Result<(), iced::Error> {
    LayersApp::run(Settings::default())
}

fn border<T: 'static>(content: Element<T>) -> container::Container<T> {

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