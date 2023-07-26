
use iced::{Sandbox, widget::{text, button, container, text_input, row, mouse_area}, Settings, Rectangle, Point, Size, Element, Background, Color, Renderer};
use iced::widget::column;

use crate::overlay_manager::*;

mod overlay_manager;

struct LayersApp {
    layers: Vec<LayerInfo>,
    dragging: Option<(i16, Point)>,
}

struct LayerInfo {
    order: i16,
    top_left: Point,
    text: String,
}

#[derive(Debug, Clone)]
enum Message {
    AddLayer,
    CloseLayer(i16),
    BringForward(i16),
    SendBack(i16),
    DragStart(i16),
    DragStop,
    SetText(i16, String)
}

impl Sandbox for LayersApp {
    type Message = Message;

    fn new() -> Self {
        Self { layers: Vec::new(), dragging: None }
    }

    fn title(&self) -> String {
        "Layers App".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::AddLayer => {
                let order = self.layers.last().map(|layer| layer.order).unwrap_or(0) + 1;
                self.layers.push(LayerInfo { 
                    order, 
                    text: "new layer".to_string(), 
                    top_left: Point::new((order * 50) as f32, (order * 50) as f32) });
            },
            Message::CloseLayer(order) => self.layers.retain(|layer| layer.order != order),
            Message::BringForward(order) => {
                let new_order = self.layers.last().map(|layer| layer.order).unwrap_or(0) + 1;
                if new_order == order + 1 {
                    return;
                }
                self.layers.iter_mut().for_each(|layer| if layer.order == order { layer.order = new_order; });
                self.layers.sort_by_key(|layer| layer.order);
            },
            Message::SendBack(order) => {
                let new_order = self.layers.first().map(|layer| layer.order).unwrap_or(1) - 1;
                if new_order == order - 1 {
                    return;
                }
                self.layers.iter_mut().for_each(|layer| if layer.order == order { layer.order = new_order; });
                self.layers.sort_by_key(|layer| layer.order);
            },
            Message::DragStart(order) => {
                let pos = self.layers.iter().find(|layer| layer.order == order).unwrap().top_left;
                self.dragging = Some((order, pos))
            },
            Message::DragStop => self.dragging = None,
            Message::SetText(order, text) => {
                self.layers.iter_mut().for_each(|layer| if layer.order == order { layer.text = text.clone(); });
            },
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {

        fn as_layer(layer: &LayerInfo, dragging: Option<(i16, Point)>) -> Layer<Message, Renderer> {
            let mut top_left = layer.top_left;
            // if let Some((order, start)) = dragging {
            //     if order == layer.order {
            //         top_left = start 
            //     }
            // }
            Layer::new(
                Rectangle::new(top_left, Size::new(300., 300.)), 
                border(column![
                    row![
                        button("^").width(30).height(30).on_press(Message::BringForward(layer.order)),
                        button("v").width(30).height(30).on_press(Message::SendBack(layer.order)),
                        button("x").width(30).height(30).on_press(Message::CloseLayer(layer.order)),
                        mouse_area(border(text("drag here").into()))
                            .on_press(Message::DragStart(layer.order)).on_release(Message::DragStop),
                    ].spacing(10),
                    text(&layer.text),
                    text_input("change text", &layer.text).on_input(|s| Message::SetText(layer.order, s))
                ].spacing(10).into()).into())
        }

        column![
            button("Add Layer").on_press_maybe(if self.layers.len() < 10 { Some(Message::AddLayer) } else { None }),
            OverlayManager::new(self.layers.iter().map(|layer| as_layer(layer, self.dragging)).collect::<Vec<Layer<Message, Renderer>>>())
        ].into()
        
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