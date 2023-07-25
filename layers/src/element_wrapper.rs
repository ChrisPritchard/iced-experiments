use iced::{Element, advanced::{Widget, self, widget::Tree, Layout, mouse, Clipboard, Shell, layout, renderer}, Rectangle, Event, event, Length};

/// A custom widget that does nothing but wrap content (another Element). Sort of a 'control' widget
pub struct ElementWrapper<'a, Message, Renderer> {
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> ElementWrapper<'a, Message, Renderer> {
    pub fn new(content: Element<'a, Message, Renderer>) -> Self {
        Self { content }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ElementWrapper<'a, Message, Renderer> 
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    // needs to be implemented for content to work
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    // needs to be implemented for content to work
    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn width(&self) -> Length {
        self.content.as_widget().width()
    }

    fn height(&self) -> Length {
        self.content.as_widget().height()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.as_widget().layout(renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as advanced::Renderer>::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(state, renderer, theme, style, layout, cursor, viewport)
    }
}

impl<'a, Message, Renderer> From<ElementWrapper<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: advanced::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(
        column: ElementWrapper<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(column)
    }
}