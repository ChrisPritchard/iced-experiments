use iced::{Element, advanced::{Widget, self, widget::{Tree, Operation}, Layout, mouse, Clipboard, Shell, layout, renderer, overlay}, Rectangle, Event, event, Length, Size};

pub struct AbsoluteContainer<'a, Message, Renderer> {
    content: Vec<(Rectangle, Element<'a, Message, Renderer>)>,
    width: Length,
    height: Length,
}

impl<'a, Message, Renderer> AbsoluteContainer<'a, Message, Renderer> {
    pub fn new(content: Vec<(Rectangle, Element<'a, Message, Renderer>)>) -> Self {
        Self { content, width: Length::Shrink, height: Length::Shrink }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for AbsoluteContainer<'a, Message, Renderer> 
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    // needs to be implemented for content to work
    fn children(&self) -> Vec<Tree> {
        self.content.iter().map(|(_, ct)| Tree::new(ct)).collect()
    }

    // needs to be implemented for content to work
    fn diff(&self, tree: &mut Tree) {
        let content: Vec<_> = self.content.iter().map(|(_, ct)| ct).collect();
        tree.diff_children(&content);
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits
            .loose()
            .width(self.width)
            .height(self.height);

        layout::Node::new(limits.max()) // might need fixing
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content
                .iter()
                .map(|(_, ct)| ct)
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                })
        });
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
        self.content.iter_mut().map(|(_, ct)| ct)
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content
            .iter()
            .map(|(_, ct)| ct)
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget().mouse_interaction(
                    state, layout, cursor, viewport, renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        for (((rect, child), state), _layout) in self.content
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            let mut node = layout::Node::new(rect.size());
            node.move_to(rect.position());
            let layout = Layout::new(&node);
            child
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, rect);
        }
    }

    // fn overlay<'b>(
    //     &'b mut self,
    //     tree: &'b mut Tree,
    //     layout: Layout<'_>,
    //     renderer: &Renderer,
    // ) -> Option<overlay::Element<'b, Message, Renderer>> {
    //     let mut content: Vec<_> = self.content.iter_mut().map(|(_, ct)| ct).collect();
    //     overlay::from_children(&mut content, tree, layout, renderer)
    // }
}

impl<'a, Message, Renderer> From<AbsoluteContainer<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: advanced::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(
        elem: AbsoluteContainer<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(elem)
    }
}