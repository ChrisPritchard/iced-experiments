use iced::{Sandbox, widget::{svg, column}, Length, Settings, Point};
use map_point::MapPoint;

mod map;
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
        // let handle = svg::Handle::from_memory(MAP_SVG_DATA);
        // svg(handle).width(Length::Fill).height(Length::Fill).into()

        column![
            MapPoint { point: Point::new(50., 50.) },
            MapPoint { point: Point::new(100., 100.) },
            MapPoint { point: Point::new(150., 150.) },
            MapPoint { point: Point::new(200., 200.) },
        ].width(Length::Fill).height(Length::Fill).into()
    }
}

fn main() -> Result<(), iced::Error> {
    WorldMap::run(Settings::default())
}
