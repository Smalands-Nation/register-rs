macro_rules! DB {
    ($fn:expr) => {
        iced::Command::perform(
            std::future::ready::<
                std::sync::Arc<
                    dyn Fn(
                            std::sync::Arc<std::sync::Mutex<rusqlite::Connection>>,
                        ) -> Result<crate::screens::Message>
                        + Send
                        + Sync,
                >,
            >(std::sync::Arc::new($fn)),
            crate::screens::Message::DB,
        );
    };
}

pub mod manager;
pub mod menu;
pub mod transactions;

use {
    crate::error::Result,
    giftwrap::Wrap,
    iced::{Command, Element},
    rusqlite::Connection,
    std::sync::{Arc, Mutex},
};
pub use {manager::Manager, menu::Menu, transactions::Transactions};

#[derive(Clone, Wrap)]
pub enum Message {
    #[noWrap]
    SwapTab(usize),
    #[noWrap]
    DB(Arc<dyn Fn(Arc<Mutex<Connection>>) -> Result<Message> + Send + Sync>),
    #[noWrap]
    CloseModal,
    Menu(menu::Message),
    Transactions(transactions::Message),
    Manager(manager::Message),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SwapTab(n) => write!(f, "SwapTab({:?})", n),
            Self::DB(_) => write!(f, "DB(_)"),
            Self::CloseModal => write!(f, "CloseModal"),
            Self::Menu(n) => write!(f, "Menu({:?})", n),
            Self::Transactions(n) => write!(f, "Transactions({:?})", n),
            Self::Manager(n) => write!(f, "Manager({:?})", n),
        }
    }
}

pub trait Screen: Sized {
    type InMessage;
    type ExMessage;

    fn new() -> (Self, Command<Self::ExMessage>);
    fn update(&mut self, msg: Self::InMessage) -> Command<Self::ExMessage>;
    fn view(&mut self) -> Element<Self::ExMessage>;
}
