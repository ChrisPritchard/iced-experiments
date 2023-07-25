// use iced_native::widget::Tree;
// use iced_native::{Widget, Element, Point};

// pub struct Layer<'a, Message, Renderer> {
//     point: Point,
//     content: Element<'a, Message, Renderer>,
// }

// impl<'a, Message, Renderer> Layer<'a, Message, Renderer> {
//     pub fn new(point: Point, content: Element<'a, Message, Renderer>) -> Self {
//         Self { point, content }
//     }
// }

// impl<'a, Message, Renderer> Widget<Message, Renderer> for Layer<'a, Message, Renderer> 
// where
//     Renderer: iced_native::Renderer,
//     Message: Clone,
// {
//     fn width(&self) -> iced_native::Length {
//         self.content.as_widget().width()
//     }

//     fn height(&self) -> iced_native::Length {
//         self.content.as_widget().height()
//     }

//     fn layout(
//         &self,
//         renderer: &Renderer,
//         limits: &iced_native::layout::Limits,
//     ) -> iced_native::layout::Node {
//         let content_layout = self.content.as_widget().layout(renderer, limits);
//         let size = limits.resolve(content_layout.size());
//         let mut layout = iced_native::layout::Node::with_children(size, vec![content_layout]);
//         layout.move_to(self.point);
//         layout
//     }

//     fn draw(
//         &self,
//         state: &Tree,
//         renderer: &mut Renderer,
//         theme: &<Renderer as iced_native::Renderer>::Theme,
//         style: &iced_native::renderer::Style,
//         layout: iced_native::Layout<'_>,
//         cursor_position: iced_native::Point,
//         viewport: &iced_native::Rectangle,
//     ) {
//         self.content.as_widget().draw(state, renderer, theme, style, layout, cursor_position, viewport)
//     }
// }

// impl<'a, Message, Renderer> From<Layer<'a, Message, Renderer>>
//     for Element<'a, Message, Renderer>
// where
//     Renderer: iced_native::Renderer + 'a,
//     Message: Clone + 'a,
// {
//     fn from(
//         column: Layer<'a, Message, Renderer>,
//     ) -> Element<'a, Message, Renderer> {
//         Element::new(column)
//     }
// }