

pub mod agent_list;
pub mod agent_details;
pub mod order_list;
pub mod order_setup;

mod prelude {
    pub use iced::{Command, Element, Length, Renderer};
    pub use iced::alignment::{Horizontal, Vertical};
    pub use iced::widget::*;

    pub use legion::{World, Entity, IntoQuery, EntityStore};

    pub use crate::world::components::*;

    pub use crate::ui::border;

    /// Creates a [Column] with the given children (re-export of [column!])
    #[macro_export(local_inner_macros)]
    macro_rules! col {
        // You can capture any arguments here if needed
        ($($args:tt)*) => {
            // Delegate the work to the original macro
            self::column!($($args)*)
        }
    }

    pub use crate::col;
}

use iced::{Theme, widget::container::Appearance};
use prelude::*;

pub fn border<T: 'static>(content: Element<T>) -> Container<T> {

    let style = |theme: &Theme| -> Appearance {
        let palette = theme.extended_palette();
        Appearance {
            border_width: 2.,
            border_color: palette.primary.base.color,
            ..Default::default()
        }
    } as for<'r> fn(&'r _) -> _;

    iced::widget::container(content)
        .padding(10)
        .style(style)
        .into()
}
