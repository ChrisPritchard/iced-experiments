
mod agency;
mod world;
mod ui;

use agency::Agency;
use iced::{Settings, Application};

fn main() -> iced::Result {
    Agency::run(Settings::default())
}
