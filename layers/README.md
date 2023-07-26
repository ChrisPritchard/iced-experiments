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

children and diff are required if the element will contain other elements, while on_event and mouse_interaction are also needed if these child elements require them

see ![./src/element_wrapper.rs](./src/element_wrapper.rs) for a complete example, basically replicating a container but almost no functionality beyond wrapping

## Overlays

The overlay function is special, and dictates that something can be drawn on top of this

It requires that it return something that implements the Overlay trait, with the following methods requiring implementation:

- layout
- draw

the layout method is used to position the content wherever, seemingly free from bounds

there are more methods with default implementations as with basic widgets:

- operate
- on_event
- mouse_interaction
- is_over <- determines if the cursor is over the content, and by default just checks if within bounds
- overlay <- for nested overlays

## Overall composition

A wrapper-like object that takes x, y, width, height
The layout, draw, mouse interaction etc methods of this wrapper do nothing
Its overlay returns an overlay component that implements the normal widget methods like a container would

while this works, there are issues: layers render over others, sure, but often rendering bleeds through since things are transparent. this doesnt occur with content and overlays - it seems as if un-nested overlays are treated as being on the same plane

## Layer Manager

Instead of having a 'layer' widget, create a 'layerS' widget that you can give a vec of layers two, with each component being treated on its own layer.
When drawn, the vec will be passed to child layers, each taking the first then pushing the rest to the next layer

there is issues with borrowing. issue being that each layer requires a mutable element and a mutable tree, as well as a slice/vec of the elements and trees to pass to the next overlay
