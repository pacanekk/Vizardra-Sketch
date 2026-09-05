mod commands;
mod message;
mod properties;
mod state;
mod tools;
mod undo;
mod update;
mod utils;
mod view;

pub use message::Message;
pub use state::AppState;

use iced::{Subscription, Theme};

pub fn run() -> Result<(), iced::Error> {
    iced::application(AppState::default, AppState::update, AppState::view)
        .title(|_state: &AppState| String::from("Vizardra Sketch"))
        .theme(|_state: &AppState| Theme::Dark)
        .subscription(AppState::subscription)
        .window_size((1280.0, 800.0))
        .run()
}

impl AppState {
    pub fn subscription(&self) -> Subscription<Message> {
        let resize = iced::window::resize_events().map(|(_id, size)| Message::WindowResized(size));
        let keys = iced::keyboard::listen().filter_map(|event| {
            if let iced::keyboard::Event::KeyPressed { key, .. } = event {
                if key == iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape) {
                    return Some(Message::EscapePressed);
                }
            }
            None
        });
        Subscription::batch([resize, keys])
    }
}
