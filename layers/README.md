# Layers

A project containing a new widget type that allows positioning elements in arbitrary places, overlapping.

Sort of a planned distillation of the modal and toast example projects into something more easily usable.

Idea will be something like this:

Normal, non overlapping:

    Root Element
        Element
            Element
            Element
        Element
        Element

New Layer Element Type:

    Root Element
        Element
        Layer Element (top_left, size)
            Element
            Element
            Element
        Layer Element
            Element
            Element
            Element
        Element

In the above, i see the second layer overlapping the first, but both overlapping elements attached to the root.

## Iced structs and traits

To implement the Layer Element, we are looking at a 'custom widget', something that implements the widget trait. This can then be used as an element for views

> An 'Element' us a generic widget, sort of like the widget interface, though more like a widget wrapper (as widget is a trait, and element is a struct). The element struct definition literally just contains a boxed widget.

Widget requires:
- width and height
- layout
- draw

a raw implementation, full of todos, would look like this - assumes adding the iced_native crate:

```rust
use iced_native::widget::Tree;
use iced_native::{Widget, Element};

pub struct Layer<'a, Message, Renderer> {
    elements: Vec<Element<'a, Message, Renderer>>
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Layer<'a, Message, Renderer> 
where
    Renderer: iced_native::Renderer,
    Message: Clone,
{
    fn width(&self) -> iced_native::Length {
        todo!()
    }

    fn height(&self) -> iced_native::Length {
        todo!()
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        todo!()
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_native::Point,
        viewport: &iced_native::Rectangle,
    ) {
        todo!()
    }
}
```

in addition, and critically for the purpose of this widget, the following methods can be overridden:

- tag
- state
- children
- diff
- operate
- on_event
- mouse_interaction
- overlay