
pub mod components;
pub mod systems;
pub mod spawner;

mod prelude {
    pub use legion::{system, World, IntoQuery, Entity, EntityStore};
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub use crate::world::components::*;
}