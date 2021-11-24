pub mod manager;
pub mod menu;
pub mod sales;
pub mod transactions;

use {
    crate::error::{Error, Result},
    giftwrap::Wrap,
    iced::{Command, Element},
    rusqlite::Connection,
    std::{
        future,
        sync::{Arc, Mutex},
    },
};
pub use {manager::Manager, menu::Menu, sales::Sales, transactions::Transactions};

#[derive(Clone, Wrap)]
pub enum Message {
    #[noWrap]
    None,
    #[noWrap]
    SwapTab(usize),
    #[noWrap]
    DB(Arc<dyn Fn(Arc<Mutex<Connection>>) -> Result<Message> + Send + Sync>),
    #[noWrap]
    CloseModal,
    Error(Error),
    Menu(menu::Message),
    Transactions(transactions::Message),
    Manager(manager::Message),
    Sales(sales::Message),
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::SwapTab(n) => write!(f, "SwapTab({:?})", n),
            Self::DB(_) => write!(f, "DB(_)"),
            Self::CloseModal => write!(f, "CloseModal"),
            Self::Error(n) => write!(f, "Error({:?})", n),
            Self::Menu(n) => write!(f, "Menu({:?})", n),
            Self::Transactions(n) => write!(f, "Transactions({:?})", n),
            Self::Manager(n) => write!(f, "Manager({:?})", n),
            Self::Sales(n) => write!(f, "Sales({:?})", n),
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

pub fn db<FN>(func: FN) -> Command<Message>
where
    FN: Fn(Arc<Mutex<Connection>>) -> Result<Message> + Send + Sync + 'static,
{
    future::ready(Message::DB(Arc::new(func))).into()
}
