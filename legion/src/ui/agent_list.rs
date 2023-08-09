

use crate::ui::prelude::*;
//use iced::widget::column;

// should list agents, scrollable
// buttons to show details or give a travel order (if not travelling) - possibly later a queue of orders or quick order might be approps?

pub struct AgentList;

#[derive(Debug, Clone)]
pub enum AgentListMessage {
    ShowDetails(Entity),
    GiveOrder(Entity),
}

impl AgentList {
    pub fn default() -> Self {
        Self { }
    }

    pub fn view(&self, ecs: &World) -> Element<AgentListMessage> {
        let agents: Vec<(&Entity, &Agent, &Info)> = <(Entity, &Agent, &Info)>::query().iter(ecs)
            .filter(|(_, a, _)| a.contractor == "Player").collect();
        
        let elements: Vec<Element<AgentListMessage>> = agents.iter().map(|(e, a, i)| {
                let order_entry = ecs.entry_ref(a.status).unwrap();
                let order = order_entry.get_component::<OrderType>().unwrap();
                row![
                    col![
                        text(&i.name).size(25),
                        text(order.description(ecs)),
                    ],                    
                    horizontal_space(Length::Fill),
                    button("info").on_press(AgentListMessage::ShowDetails(**e)),
                    button("order").on_press(AgentListMessage::GiveOrder(**e)),
                    horizontal_space(Length::Fixed(10.))
                ].spacing(10).into()
            }).collect();

        border(
            scrollable(
                column(elements).spacing(10)
            ).into()
        ).into()
    }
}
