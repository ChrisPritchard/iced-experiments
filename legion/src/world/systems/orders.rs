use crate::world::prelude::*;

#[system]
#[read_component(Agent)]
#[read_component(OrderType)]
pub fn orders(ecs: &mut SubWorld, command: &mut CommandBuffer) {
    <(Entity, &Agent)>::query().iter(ecs).for_each(|(e, a)| {
        let order = ecs.entry_ref(a.status).unwrap();

        if let OrderType::Travelling(info) = order.get_component::<OrderType>().unwrap() {
            let new_order = OrderType::Idle(info.to_city);
            new_order.replace_order_by_command(*e, ecs, command);
        }
    });
}