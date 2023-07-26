use iced::{Element, advanced::{Widget, self, widget::{Tree, self}, Layout, mouse, Clipboard, Shell, layout, renderer, overlay}, Rectangle, Event, event, Length, Size, Point, Alignment};

pub struct Layer<'a, Message, Renderer> {
    rect: Rectangle<f32>,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Layer<'a, Message, Renderer> {
    pub fn new(rect: Rectangle<f32>, content: Element<'a, Message, Renderer>) -> Self {
        Self { rect, content }
    }
}

// largely does nothing - no size, no drawing, no other fancy event methods. just creates the overlay on request, passing in the position and size variables from the struct

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

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut widget::Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        Some(overlay::Element::new(
            self.rect.position(),
            Box::new(LayerOverlay {
                content: &mut self.content,
                tree: &mut state.children[0],
                size: self.rect.size(),
            }),
        ))
    }
}

/// A very basic overlay implementation that operates similar to a container, 
/// deferring most function calls to the content it holds
struct LayerOverlay<'a, 'b, Message, Renderer> {
    content: &'b mut Element<'a, Message, Renderer>,
    tree: &'b mut widget::Tree,
    size: Size,
}

impl<'a, 'b, Message, Renderer> overlay::Overlay<Message, Renderer>
        for LayerOverlay<'a, 'b, Message, Renderer>
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    fn layout(
        &self,
        renderer: &Renderer,
        _bounds: Size,
        position: Point,
    ) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);

        let mut child = self.content.as_widget().layout(renderer, &limits);
        child.align(Alignment::Center, Alignment::Center, limits.max());

        let mut node = layout::Node::with_children(self.size, vec![child]);
        node.move_to(position);

        node
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            self.tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.content.as_widget().operate(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Renderer>> {
        self.content.as_widget_mut().overlay(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
        )
    }
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