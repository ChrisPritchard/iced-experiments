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

pub struct LayerManager<'a, Message, Renderer> {
    content: Vec<Layer<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> LayerManager<'a, Message, Renderer> {
    pub fn new(content: Vec<Layer<'a, Message, Renderer>>) -> Self {
        Self { content }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for LayerManager<'a, Message, Renderer> 
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

    fn children(&self) -> Vec<widget::Tree> {
        self.content.iter().map(|ld| widget::Tree::new(&ld.content)).collect()
    }

    fn diff(&self, tree: &mut widget::Tree) {
        let elems = self.content.iter().map(|ld| &ld.content).collect::<Vec<&Element<Message, Renderer>>>();
        tree.diff_children(&elems);
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut widget::Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        if self.content.is_empty() {
            return None
        }
        
        let (first, rest) = self.content.split_first_mut().unwrap();
        let (first_tree, forest) = state.children.split_first_mut().unwrap();

        Some(overlay::Element::new(
            first.rect.position(),
            Box::new(LayerOverlay {
                content: &mut first.content,
                layers: rest,
                tree: first_tree,
                trees: forest,
                rect: first.rect,
            }),
        ))
    }
}

struct LayerOverlay<'a, 'b, Message, Renderer> {
    content: &'b mut Element<'a, Message, Renderer>,
    layers: &'b mut [Layer<'a, Message, Renderer>],
    tree: &'b mut widget::Tree,
    trees: &'b mut [widget::Tree],
    rect: Rectangle<f32>,
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
        let limits = layout::Limits::new(Size::ZERO, self.rect.size())
            .width(Length::Fill)
            .height(Length::Fill);

        let mut child = self.content.as_widget().layout(renderer, &limits);
        child.align(Alignment::Center, Alignment::Center, limits.max());

        let mut node = layout::Node::with_children(child.size(), vec![child]);
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

    fn overlay<'c>(
        &'c mut self,
        _layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Renderer>> {
        if self.layers.is_empty() {
            return None;
        }

        let (first, rest) = self.layers.split_first_mut().unwrap();
        let (first_tree, forest) = self.trees.split_first_mut().unwrap();

        Some(overlay::Element::new(
            first.rect.position(),
            Box::new(LayerOverlay {
                content: &mut first.content,
                layers: rest,
                tree: first_tree,
                trees: forest,
                rect: first.rect,
            }),
        ))
    }
}

impl<'a, Message, Renderer> From<LayerManager<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: advanced::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(
        elem: LayerManager<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(elem)
    }
}