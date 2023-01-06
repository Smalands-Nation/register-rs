pub mod info;
pub mod manager;
pub mod menu;
pub mod sales;
pub mod transactions;

use {
    crate::{
        error::{Error, Result},
        Element,
    },
    giftwrap::Wrap,
    iced::Command,
};
pub use {info::Info, manager::Manager, menu::Menu, sales::Sales, transactions::Transactions};

#[derive(Clone, Wrap, Debug)]
pub enum Message {
    #[giftwrap(noWrap = true)]
    None,
    #[giftwrap(noWrap = true)]
    SwapTab(usize),
    #[giftwrap(noWrap = true)]
    CloseModal,
    Error(Error),
    Menu(menu::Message),
    Transactions(transactions::Message),
    Manager(manager::Message),
    Sales(sales::Message),
    Info(info::Message),
}

impl From<()> for Message {
    fn from(_: ()) -> Self {
        Self::None
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
    fn view(&self) -> Element<Self::ExMessage>;
}

#[macro_export]
macro_rules! sql {
    ($sql:literal, $params:expr, $msg:expr) => {
        $crate::command!({
            $crate::DB.lock().await.execute($sql, $params)?;
            Ok($msg)
        })
    };

    ($sql:literal, $params:expr, $map_row:expr, $collect:ty, $msg:expr) => {
        $crate::command!({
            Ok($msg(
                $crate::DB
                    .lock()
                    .await
                    .prepare($sql)?
                    .query_map($params, $map_row)?
                    .collect::<std::result::Result<$collect, rusqlite::Error>>()?,
            ))
        })
    };
}
