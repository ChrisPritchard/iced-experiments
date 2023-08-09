use legion::{Entity, World, EntityStore, systems::CommandBuffer, world::SubWorld};


#[derive(Clone, Debug, PartialEq)]
pub struct Agent {
    pub contractor: String,
    pub status: OrderEntity,
}

pub type OrderEntity = Entity;

impl Agent {
    pub fn new(contractor: String, initial_status: OrderEntity) -> Self {
        Self { contractor, status: initial_status }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct City;

#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Flight {
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Progress {
    pub percentage: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMessage {
    pub text: String
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderInfo {
    pub next_order: Option<Entity>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OrderType {
    Idle(CityEntity),
    Travelling(TravelInfo)
}

pub type CityEntity = Entity;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TravelInfo {
    pub from_city: CityEntity,
    pub to_city: CityEntity,
}

impl OrderType {
    pub fn description(&self, ecs: &World) -> String {
        match self {
            OrderType::Idle(city_ent) => format!("Idle in {}", ecs.entry_ref(*city_ent).unwrap().get_component::<Info>().unwrap().name),
            OrderType::Travelling(travel_info) => {
                let from = ecs.entry_ref(travel_info.from_city).unwrap().get_component::<Info>().unwrap().name.clone();
                let to = ecs.entry_ref(travel_info.to_city).unwrap().get_component::<Info>().unwrap().name.clone();
                format!("Travelling from {} to {}", from, to)
            }
        }
    }

    pub fn for_agent(agent_entity: Entity, ecs: &World) -> Self {
        let agent_entry = ecs.entry_ref(agent_entity).unwrap();
        let order_entity = agent_entry.get_component::<Agent>().unwrap().status;
        *ecs.entry_ref(order_entity).unwrap().get_component::<OrderType>().unwrap()
    }

    pub fn replace_order(&self, agent: Entity, ecs: &mut World) {
        let new_order = (OrderInfo{next_order: None}, self.clone());
        let new_order_entity = ecs.push(new_order);
    
        let mut agent_entry = ecs.entry_mut(agent).unwrap();
        let mut agent = agent_entry.get_component_mut::<Agent>().unwrap();

        let old_order_entity = agent.status;
        agent.status = new_order_entity;
    
        ecs.remove(old_order_entity);
    }

    pub fn replace_order_by_command(&self, agent_entity: Entity, ecs: &SubWorld, commands: &mut CommandBuffer) {
        let new_order = (OrderInfo{next_order: None}, self.clone());
        let new_order_entity = commands.push(new_order);

        let agent_entry = ecs.entry_ref(agent_entity).unwrap();
        let agent = agent_entry.get_component::<Agent>().unwrap();

        let old_order_entity = agent.status;
        commands.remove(old_order_entity);

        let mut agent = agent.clone();
        agent.status = new_order_entity;
        commands.add_component(agent_entity, agent);
    }
}


impl TravelInfo {
    pub fn from_agent_idle(agent: Entity, dest: Entity, ecs: &World) -> TravelInfo {
        let current_order = OrderType::for_agent(agent, ecs);
        let from_city = match current_order {
            OrderType::Idle(c) => c,
            _ => panic!(),
        };
        TravelInfo{from_city,to_city: dest}
    }
}