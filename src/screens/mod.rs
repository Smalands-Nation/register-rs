pub mod manager;
pub mod menu;

use {
    crate::{error::Result, Marc},
    iced::{Command, Element},
    rusqlite::Connection,
};
pub use {manager::Manager, menu::Menu};

#[derive(Debug, Clone)]
pub enum Message {
    SwapTab(usize),
    ReadDB(fn(Marc<Connection>) -> Result<Message>),
    WriteDB(String),
    CloseModal,
    Menu(menu::Message),
    Manager(manager::Message),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Screen: Sized {
    type InMessage;
    type ExMessage;

    fn new() -> (Self, Command<Self::ExMessage>);
    fn update(&mut self, msg: Self::InMessage) -> Command<Self::ExMessage>;
    fn view(&mut self) -> Element<Self::ExMessage>;
}
