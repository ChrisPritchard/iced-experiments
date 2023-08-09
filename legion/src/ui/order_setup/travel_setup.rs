

use std::collections::HashMap;

use crate::ui::prelude::*;

pub struct TravelSetup {
    city_map: HashMap<String, Entity>,
    current_city: String,
    destinations: Vec<String>,
    current_dest: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TravelSetupMessage {
    Cancel,
    SelectCity(String),
    Confirm(Entity),
}

type Message = TravelSetupMessage;

impl TravelSetup {

    pub fn new(current_city: String, destinations: Vec<(Entity, String)>) -> Self {
        let city_map: HashMap<String, Entity> = destinations.iter().map(|(e, n)| (n.clone(), *e)).collect(); // possibly custom display type?
        let destinations: Vec<String> = destinations.iter().map(|(_, n)| n.clone()).collect();
        Self { city_map, current_city, destinations, current_dest: None }
    }

    pub fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::SelectCity(c) => self.current_dest = Some(c),
            _ => () // handled by parent
        }
        Command::none()
    }

    pub fn view(&self, _world: &World) -> Element<Message> {

        let mut controls: Vec<Element<Message>> = Vec::new();
        controls.push(button("cancel").on_press(Message::Cancel).into());
        controls.push(horizontal_space(Length::Fill).into());

        if self.current_dest.is_some() {
            let city_name = self.current_dest.as_ref().unwrap().clone();
            controls.push(button("confirm").on_press(Message::Confirm(self.city_map[&city_name])).into());
        }        
        
        let current_dest = if self.current_dest.is_some() { Some(self.current_dest.as_ref().unwrap().clone()) } else { None };

        col![
            row![
                col![
                    text("Current Location:"),
                    text("New Destination:"),
                ].spacing(25).width(Length::FillPortion(1)),
                col![
                    text(self.current_city.clone()),
                    pick_list(&self.destinations, current_dest, Message::SelectCity),
                ].spacing(20).width(Length::FillPortion(2)),
            ].width(Length::Fill),
            horizontal_rule(2),
            row(controls),
        ].spacing(10).into()
    }
}