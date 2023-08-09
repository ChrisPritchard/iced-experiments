use crate::ui::prelude::*;

pub struct AgentDetails {
    pub agent: Entity
}

#[derive(Debug, Clone)]
pub enum AgentDetailsMessage {
    Close, GiveOrder
}

impl AgentDetails {
    pub fn default(agent: Entity) -> Self {
        Self { agent }
    }

    pub fn view(&self, ecs: &World) -> Element<AgentDetailsMessage> {
        let agent_entry = ecs.entry_ref(self.agent).unwrap();
        let order_ent = agent_entry.get_component::<Agent>().unwrap().status;
        let order_entry = ecs.entry_ref(order_ent).unwrap();
        let order = order_entry.get_component::<OrderType>().unwrap();
        let info = agent_entry.get_component::<Info>().unwrap();
        
        border(col![
            text(&info.name).size(30),
            horizontal_rule(2),
            text(order.description(ecs)),
            horizontal_rule(2),
            text(&info.description),
            horizontal_rule(2),
            row![
                button("close").on_press(AgentDetailsMessage::Close),
                button("give order").on_press(AgentDetailsMessage::GiveOrder),
                horizontal_space(Length::Fill),
            ].spacing(10)
        ]
        .spacing(10)
        .width(400)
        .into()).into()
    }
}
