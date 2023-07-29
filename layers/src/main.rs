
use iced::{widget::{text, button, container, text_input, row, mouse_area}, Settings, Rectangle, Point, Size, Element, Background, Color, Renderer, Application, Command, subscription, Subscription, Event, advanced::mouse, executor, Theme};
use iced::widget::column;

use crate::overlay_manager::*;

mod overlay_manager;

struct LayersApp {
    layers: Vec<LayerInfo>,
    dragging: Option<(i16, Option<Point>)>,
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
    SetText(i16, String),
    EventOccurred(Event),
}

impl Application for LayersApp {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self { layers: Vec::new(), dragging: None },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "Layers App".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
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
                    return Command::none();
                }
                self.layers.iter_mut().for_each(|layer| if layer.order == order { layer.order = new_order; });
                self.layers.sort_by_key(|layer| layer.order);
            },
            Message::SendBack(order) => {
                let new_order = self.layers.first().map(|layer| layer.order).unwrap_or(1) - 1;
                if new_order == order - 1 {
                    return Command::none();
                }
                self.layers.iter_mut().for_each(|layer| if layer.order == order { layer.order = new_order; });
                self.layers.sort_by_key(|layer| layer.order);
            },
            Message::DragStart(order) => {
                self.dragging = Some((order, None))
            },
            Message::DragStop => self.dragging = None,
            Message::SetText(order, text) => {
                self.layers.iter_mut().for_each(|layer| if layer.order == order { layer.text = text.clone(); });
            },

            Message::EventOccurred(evt) => {
                if self.dragging.is_some() {
                    if let Event::Mouse(em) = evt {
                        if let mouse::Event::CursorMoved { position } = em {
                            let (order, old_position) = self.dragging.unwrap();
                            if old_position.is_some() {
                                let old_position = old_position.unwrap();
                                let diff = position - old_position;
                                self.layers.iter_mut().for_each(|layer| if layer.order == order { layer.top_left = layer.top_left + diff; });
                            }
                            self.dragging = Some((order, Some(position)));
                        }
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {

        fn as_layer(layer: &LayerInfo) -> Layer<Message, Renderer> {
            Layer::new(
                Rectangle::new(layer.top_left, Size::new(300., 300.)), 
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
            OverlayManager::new(self.layers.iter().map(|layer| as_layer(layer)).collect::<Vec<Layer<Message, Renderer>>>())
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