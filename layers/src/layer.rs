use iced::{Element, advanced::{Widget, self, widget::Tree, Layout, mouse, Clipboard, Shell, layout, renderer}, Rectangle, Event, event, Length, Size};

pub struct Layer<'a, Message, Renderer> {
    rect: Rectangle<f32>,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Layer<'a, Message, Renderer> {
    pub fn new(rect: Rectangle<f32>, content: Element<'a, Message, Renderer>) -> Self {
        Self { rect, content }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Layer<'a, Message, Renderer> 
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::ZERO)
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _renderer_style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) { }
}

impl<'a, Message, Renderer> From<Layer<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: advanced::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(
        elem: Layer<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(elem)
    }
}