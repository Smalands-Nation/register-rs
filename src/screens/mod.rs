pub mod manager;
pub mod menu;
pub mod sales;
pub mod transactions;

use {
    crate::error::{Error, Result},
    giftwrap::Wrap,
    iced::{Command, Element},
};
pub use {manager::Manager, menu::Menu, sales::Sales, transactions::Transactions};

#[derive(Clone, Wrap)]
pub enum Message {
    #[noWrap]
    None,
    #[noWrap]
    SwapTab(usize),
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
            Self::CloseModal => write!(f, "CloseModal"),
            Self::Error(n) => write!(f, "Error({:?})", n),
            Self::Menu(n) => write!(f, "Menu({:?})", n),
            Self::Transactions(n) => write!(f, "Transactions({:?})", n),
            Self::Manager(n) => write!(f, "Manager({:?})", n),
            Self::Sales(n) => write!(f, "Sales({:?})", n),
        }
    }
}

impl<T> From<Result<T>> for Message
where
    T: Into<Message>,
{
    fn from(r: Result<T>) -> Self {
        match r {
            Ok(t) => t.into(),
            Err(e) => e.into(),
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

#[macro_export]
macro_rules! sql {
    ($sql:literal, $params:expr, $msg:expr) => {
        Command::perform(
            async move {
                crate::DB.lock().await.execute($sql, $params)?;
                Ok($msg)
            },
            crate::screens::Message::from,
        )
    };

    ($sql:literal, $params:expr, $map_row:expr, $collect:ty, $msg:expr) => {
        Command::perform(
            async move {
                Ok($msg(
                    crate::DB
                        .lock()
                        .await
                        .prepare($sql)?
                        .query_map($params, $map_row)?
                        .collect::<std::result::Result<$collect, rusqlite::Error>>()?,
                ))
            },
            crate::screens::Message::from,
        )
    };
}
