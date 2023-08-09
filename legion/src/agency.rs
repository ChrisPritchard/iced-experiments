use std::collections::HashMap;

use iced::{Application, executor, Command, Length, Element};
use iced::widget::*;
use iced::widget::column;
use legion::{Entity, World, IntoQuery, Schedule, Resources};

use crate::world::systems::build_scheduler;
use crate::{ui::{
    agent_list::{AgentList, AgentListMessage}, 
    agent_details::{AgentDetails, AgentDetailsMessage}, 
    order_list::{OrderList, OrderListMessage}, order_setup::travel_setup::TravelSetupMessage}, 
    world::{spawner, components::*}};

pub struct Agency {
    // world
    world: World,

    schedule: Schedule,
    resources: Resources,

    // various ui elements, behind option types
    agent_list: Option<AgentList>,
    agent_details: Option<AgentDetails>,
    order_list: Option<OrderList>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AgentList(AgentListMessage),
    AgentDetails(AgentDetailsMessage),
    OrderList(OrderListMessage),

    EndTurn,
}

impl Application for Agency {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let world = spawner::new_world();
        let schedule = build_scheduler();
        let resources = Resources::default();
        (
            Self {
                world,
                schedule,
                resources,

                agent_list: Some(AgentList::default()),
                agent_details: None,
                order_list: None
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "Agency".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::AgentList(AgentListMessage::GiveOrder(agent)) => {
                let agents: HashMap<Entity, String> = <(Entity, &Agent, &Info)>::query().iter(&self.world)
                    .filter_map(|(e, a, i)| if a.contractor == "Player" { Some((*e, i.name.to_string())) } else { None })
                    .collect();
                self.order_list = Some(OrderList::new(agents, agent));
            },
            Message::AgentList(AgentListMessage::ShowDetails(agent)) => self.agent_details = Some(AgentDetails::default(agent)),
            Message::AgentDetails(AgentDetailsMessage::Close) => self.agent_details = None,
            Message::AgentDetails(AgentDetailsMessage::GiveOrder) => {
                let agent = self.agent_details.as_ref().unwrap().agent;
                let agents: HashMap<Entity, String> = <(Entity, &Agent, &Info)>::query().iter(&self.world)
                    .filter_map(|(e, a, i)| if a.contractor == "Player" { Some((*e, i.name.to_string())) } else { None })
                    .collect();
                self.order_list = Some(OrderList::new(agents, agent));
            },
            Message::OrderList(OrderListMessage::Close) => self.order_list = None,
            Message::OrderList(OrderListMessage::TravelSetup(TravelSetupMessage::Confirm(to_city))) => {
                let agent = self.order_list.as_ref().unwrap().current_agent;
                let travel_info = TravelInfo::from_agent_idle(agent, to_city, &self.world);
                let new_order = OrderType::Travelling(travel_info);
                new_order.replace_order(agent, &mut self.world);
                self.order_list = None;

            }
            Message::OrderList(m) => return self.order_list.as_mut().unwrap().update(m, &self.world).map(Message::OrderList),

            Message::EndTurn => self.schedule.execute(&mut self.world, &mut self.resources), // might require pre-validation
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let mut windows: Vec<Element<Message>> = Vec::new();

        fn as_width(child: Element<Message>, portion: u16) -> Element<Message> {
            container(child).width(Length::FillPortion(portion)).into()
        }

        if let Some(a) = &self.agent_list {
            let elem = a.view(&self.world).map(Message::AgentList);
            windows.push(as_width(elem, 1));
        }

        if let Some(a) = &self.agent_details {
            let elem = a.view(&self.world).map(Message::AgentDetails);
            windows.push(as_width(elem, 1));
        }

        if let Some(a) = &self.order_list {
            let elem = a.view(&self.world).map(Message::OrderList);
            windows.push(as_width(elem, 1));
        }

        column![
            row(windows).padding(10).spacing(10)
                .height(Length::FillPortion(7)),
            row![
                horizontal_space(Length::Fill),
                column![
                    vertical_space(Length::Fill),
                    button(text("End Turn").size(40)).on_press(Message::EndTurn),
                    vertical_space(Length::Fill),
                ]
            ]
                .height(Length::FillPortion(1))
                .padding(10)
        ].spacing(10).into()
    }
}