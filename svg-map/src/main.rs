use iced::{Sandbox, widget::svg, Length, Settings};

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
    }
}

fn main() -> Result<(), iced::Error> {
    WorldMap::run(Settings::default())
}
