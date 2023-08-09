

use std::{collections::HashMap, fmt::Display};

use crate::ui::prelude::*;

use super::order_setup::travel_setup::*;

#[derive(Debug, Clone)]
pub enum OrderOption {
    Travel
}

impl Display for OrderOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderOption::Travel => f.write_str("Travel"),
        }
    }
}

pub struct OrderList {
    agent_map: HashMap<Entity, String>,
    pub current_agent: Entity,
    
    travel_order: Option<TravelSetup>,
}

#[derive(Debug, Clone)]
pub enum OrderListMessage {
    Close,
    SelectAgent(String),
    SelectOrder(OrderOption),

    TravelSetup(TravelSetupMessage),
}

type Message = OrderListMessage;

impl OrderList {

    pub fn new(agent_map: HashMap<Entity, String>, initial_agent: Entity) -> Self {
        Self { agent_map, current_agent: initial_agent, travel_order: None }
    }

    pub fn update(&mut self, message: Message, ecs: &World) -> iced::Command<Message> {
        match message {
            Message::SelectAgent(agent_name) => {
                let ent = self.agent_map.iter().find_map(|(id, nm)| if nm == &agent_name { Some(id) } else { None });
                self.current_agent = *ent.unwrap();
                self.travel_order = None;
            },

            Message::SelectOrder(o) => {
                match o {
                    OrderOption::Travel => {
                        let order = OrderType::for_agent(self.current_agent, ecs);

                        let agent_location = match order { OrderType::Idle(c) => c, _ => panic!() };
                        let agent_location_name = ecs.entry_ref(agent_location).unwrap().get_component::<Info>().unwrap().name.clone();
                        let destinations: Vec<(Entity, String)> = <(Entity, &City, &Info)>::query().iter(ecs).filter_map(|(e, _, i)| if *e != agent_location { Some((*e, i.name.clone())) } else { None }).collect();
                        
                        let travel_order = TravelSetup::new(agent_location_name, destinations);
                        self.travel_order = Some(travel_order);
                    },
                }
            },

            Message::TravelSetup(TravelSetupMessage::Cancel) => self.travel_order = None,
            Message::TravelSetup(m) => {
                return self.travel_order.as_mut().unwrap().update(m).map(Message::TravelSetup);
            },

            _ => () // handled by parent, including travel order confirm
        }
        Command::none()
    }

    pub fn view(&self, ecs: &World) -> Element<Message> {

        let agent_options: Vec<String> = self.agent_map.iter().map(|(_, n)| n.clone()).collect();
        let current_agent = self.agent_map[&self.current_agent].clone();

        // get possible orders for agent... to begin with this is just relocate (assuming not already travelling)
        let mut possible_orders: Vec<OrderOption> = Vec::new();

        let order = OrderType::for_agent(self.current_agent, ecs);
        if let OrderType::Idle(_) = order {
            possible_orders.push(OrderOption::Travel);
        }

        let order_elems: Vec<Element<OrderListMessage>> = 
            possible_orders.iter().map(|o| {
                button(text(o)).on_press(OrderListMessage::SelectOrder(o.clone())).into()
            }).collect();

        border(
            col![
                row![
                    text("Orders").size(50).vertical_alignment(Vertical::Center),
                    horizontal_space(Length::Fill),
                    button("close").on_press(OrderListMessage::Close)
                ].height(50),
                row![
                    text("Agent").height(Length::Fill).vertical_alignment(Vertical::Center),
                    pick_list(agent_options, Some(current_agent), OrderListMessage::SelectAgent),
                ].spacing(10).height(30),
                horizontal_rule(2),
                if self.travel_order.is_some() {
                    self.travel_order.as_ref().unwrap().view(ecs).map(Message::TravelSetup)
                } else {
                    if possible_orders.len() == 0 {
                        col![
                            text("No orders currently available for this agent")
                        ]
                    } else {
                        column(order_elems)
                    }.into()
                }
            ].spacing(10).into()
        ).into()
    }
}
