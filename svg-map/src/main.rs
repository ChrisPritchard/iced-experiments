use iced::{Sandbox, widget::{svg, column, text}, Length, Settings, Point, Rectangle, Size};
use map_point::MapPoint;

mod overlay_manager;
mod absolute_container;
mod map_point;

const MAP_SVG_DATA: &[u8] = include_bytes!("../resources/world_map.svg");

struct WorldMap {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for WorldMap {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("SVG Map")
    }

    fn update(&mut self, _message: Self::Message) {
        
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let handle = svg::Handle::from_memory(MAP_SVG_DATA);
        svg(handle).width(Length::Fill).height(Length::Fill).into()

        // absolute_container::AbsoluteContainer::new(vec![
        //     (Rectangle::new(Point::new(10., 10.), Size::new(30., 30.)), text("1").into()),
        //     (Rectangle::new(Point::new(100., 100.), Size::new(30., 30.)), text("2").into()),
        //     (Rectangle::new(Point::new(300., 300.), Size::new(30., 30.)), text("3").into()),
        //     (Rectangle::new(Point::new(350., 350.), Size::new(30., 30.)), text("4").into()),
        // ]).width(Length::Fill).height(Length::Fill).into()
    }
}

fn main() -> Result<(), iced::Error> {
    WorldMap::run(Settings::default())
}
