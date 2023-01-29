pub mod info;
//pub mod manager;
pub mod menu;
//pub mod sales;
pub mod transactions;

use {
    crate::{
        error::{Error, Result},
        item::{kind::Sales, Item},
        payment::Payment,
        Element,
    },
    chrono::{DateTime, Local},
    futures::{future::BoxFuture, FutureExt},
    giftwrap::Wrap,
    iced::Command,
    rusqlite::params,
    std::{
        future::{Future, IntoFuture},
        sync::Arc,
    },
};

use {info::Info, menu::Menu, transactions::Transactions};

#[macro_export]
macro_rules! sql {
    ($sql:literal, $params:expr, $map_row:expr, $collect:ty) => {
        $crate::DB
            .lock()
            .await
            .prepare($sql)?
            .query_map($params, $map_row)?
            .collect::<std::result::Result<$collect, rusqlite::Error>>()?
    };

    ($sql:literal, $params:expr, $map_row:expr, $collect:ty) => {
        sql!($sql, ::rusqlite::params![], $map_row, $collect)
    };
}

#[derive(Clone, Debug)]
pub enum Tab {
    Menu(Vec<Item<Sales>>),
    Transactions(Vec<(DateTime<Local>, Item<Sales>, Payment)>),
    Info(self_update::Status),
}

impl From<&Tab> for usize {
    fn from(value: &Tab) -> Self {
        match value {
            Tab::Menu(_) => 0,
            Tab::Transactions(_) => 1,
            Tab::Info(_) => 2,
        }
    }
}

impl From<usize> for Tab {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Menu(vec![]),
            1 => Self::Transactions(vec![]),
            2 => Self::Info(self_update::Status::UpToDate("".into())),
            n => unreachable!("Tab {} does not exist", n),
        }
    }
}

impl Tab {
    pub fn as_menu(&self) -> Element<Message> {
        if let Self::Menu(menu) = self {
            Menu::new(menu.clone(), Message::Sideffect).into()
        } else {
            iced::widget::Text::new("Empty").into()
        }
    }

    pub fn as_transactions(&self) -> Element<Message> {
        if let Self::Transactions(transactions) = self {
            Transactions::new(transactions.clone(), Message::Sideffect).into()
        } else {
            iced::widget::Text::new("Empty").into()
        }
    }

    pub fn as_info(&self) -> Element<Message> {
        if let Self::Info(ver) = self {
            Info::new(ver.clone()).into()
        } else {
            iced::widget::Text::new("Empty").into()
        }
    }

    pub fn load(self) -> Command<Message> {
        crate::command!({
            Ok(Message::LoadTab(match self {
                Self::Menu(_) => Self::Menu(sql!(
                    "SELECT name, price, special, category FROM menu
                    WHERE available=true
                    ORDER BY
                        special ASC,
                        CASE category
                            WHEN 'alcohol' THEN 1
                            WHEN 'drink' THEN 2
                            WHEN 'food' THEN 3
                            WHEN 'other' THEN 4
                            ELSE 5
                        END,
                        name DESC",
                    params![],
                    Item::new_menu,
                    Vec<_>
                )),

                Self::Transactions(_) => Self::Transactions(sql!(
                    "SELECT * FROM receipts_view \
                    WHERE time > date('now','-1 day') ORDER BY time DESC",
                    params![],
                    |row| {
                        Ok((
                            row.get::<_, DateTime<Local>>("time")?,
                            Item::new_sales(row)?,
                            Payment::try_from(row.get::<usize, String>(5)?).unwrap_or_default(),
                        ))
                    },
                    Vec<_>
                )),

                Self::Info(_) => Self::Info(crate::config::update()?),
            }))
        })
    }
}

#[derive(Clone)]
pub struct Sideffect(futures::future::Shared<BoxFuture<'static, Result<()>>>);

impl Sideffect {
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        Self(f().boxed().shared())
    }
}

impl IntoFuture for Sideffect {
    type Output = Result<()>;
    type IntoFuture = futures::future::Shared<BoxFuture<'static, Result<()>>>;

    fn into_future(self) -> Self::IntoFuture {
        self.0
    }
}

impl std::fmt::Debug for Sideffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sideffect(_)")
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    None,
    SwapTab(Tab),
    LoadTab(Tab),
    CloseModal,
    Error(Error),
    Sideffect(Sideffect),
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
            Err(e) => Self::Error(e),
        }
    }
}
