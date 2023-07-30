use iced::{Point, advanced::{Widget, self, layout, renderer}, Size, Color, Element};

const MAP_POINT_SIZE: f32 = 32.;

pub struct MapPoint {
    pub point: Point
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for MapPoint 
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    fn width(&self) -> iced::Length {
        iced::Length::Fixed(MAP_POINT_SIZE)
    }

    fn height(&self) -> iced::Length {
        iced::Length::Fixed(MAP_POINT_SIZE)
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let mut node = layout::Node::new(Size::new(MAP_POINT_SIZE, MAP_POINT_SIZE));
        node.move_to(self.point);

        node
    }

    fn draw(
        &self,
        _state: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &<Renderer as iced::advanced::Renderer>::Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: (MAP_POINT_SIZE*2.).into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color::from_rgb(1., 0., 0.),
        );
    }
}

impl<'a, Message, Renderer> From<MapPoint>
    for Element<'a, Message, Renderer>
where
    Renderer: advanced::Renderer + 'a,
    Message: Clone + 'a,
{
    fn from(
        elem: MapPoint,
    ) -> Element<'a, Message, Renderer> {
        Element::new(elem)
    }
}