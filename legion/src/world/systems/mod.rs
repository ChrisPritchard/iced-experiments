mod orders;

use legion::Schedule;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(orders::orders_system())
        .build()
}