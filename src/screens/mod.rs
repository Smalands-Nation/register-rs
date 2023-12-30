pub mod info;
pub mod manager;
pub mod menu;
pub mod sales;
pub mod transactions;

use {
    crate::{
        error::{Error, Result},
        item::Item,
        payment::Payment,
        Element,
    },
    chrono::{Date, DateTime, Local},
    futures::{future::BoxFuture, FutureExt},
    rusqlite::params,
    std::future::{Future, IntoFuture},
};

use {info::Info, manager::Manager, menu::Menu, sales::Sales, transactions::Transactions};

#[macro_export]
macro_rules! sql {
    ($sql:literal, $params:expr, $map_row:expr, $collect:ty) => {
        $crate::DB
            .lock()
            .await
            .prepare($sql)?
            .query_map($params, $map_row)?
            .collect::<Result<$collect, rusqlite::Error>>()?
    };

    ($sql:literal, $params:expr, $map_row:expr, $collect:ty) => {
        sql!($sql, ::rusqlite::params![], $map_row, $collect)
    };
}

#[derive(Clone, Debug)]
pub enum Tab {
    Menu(Vec<Item>),
    Transactions(Vec<(DateTime<Local>, Item, Payment)>),
    Sales {
        from: Date<Local>,
        to: Date<Local>,
        data: Vec<(Item, Payment)>,
    },
    Manager(Vec<Item>),
    Info(self_update::Status),
}

impl Tab {
    pub fn as_menu(&self) -> Element<Message> {
        if let Self::Menu(menu) = self {
            Menu::new(menu.clone()).into()
        } else {
            iced::widget::Text::new("Empty").into()
        }
    }

    pub fn as_transactions(&self) -> Element<Message> {
        if let Self::Transactions(transactions) = self {
            Transactions::new(transactions.clone()).into()
        } else {
            iced::widget::Text::new("Empty").into()
        }
    }

    pub fn as_sales(&self) -> Element<Message> {
        if let Self::Sales { from, to, data } = self {
            Sales::new(*from, *to, data.clone()).into()
        } else {
            iced::widget::Text::new("Empty").into()
        }
    }

    pub fn as_manager(&self) -> Element<Message> {
        if let Self::Manager(menu) = self {
            Manager::new(menu.clone()).into()
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

    pub fn id(&self) -> TabId {
        match self {
            Self::Menu(_) => TabId::Menu,
            Self::Transactions(_) => TabId::Transactions,
            Self::Sales { from, to, .. } => TabId::Sales {
                from: from.clone(),
                to: to.clone(),
            },
            Self::Manager(_) => TabId::Manager,
            Self::Info(_) => TabId::Info,
        }
    }
}

#[derive(Clone, Debug, Eq)]
pub enum TabId {
    Menu,
    Transactions,
    Sales { from: Date<Local>, to: Date<Local> },
    Manager,
    Info,
}

impl PartialEq for TabId {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Menu, Self::Menu)
            | (Self::Transactions, Self::Transactions)
            | (Self::Sales { .. }, Self::Sales { .. })
            | (Self::Manager, Self::Manager)
            | (Self::Info, Self::Info) => true,
            _ => false,
        }
    }
}

impl TabId {
    pub async fn load(self) -> Result<Message> {
        Ok(Message::LoadTab(match self {
            Self::Menu => Tab::Menu(sql!(
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

            Self::Transactions => Tab::Transactions(sql!(
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

            Self::Sales { from, to } => {
                let from_time = from.and_hms(0, 0, 0);
                let to_time = to.and_hms(23, 59, 59);
                Tab::Sales {
                    from,
                    to,
                    data: sql!(
                        "SELECT item, amount, price, special, method FROM receipts_view \
                            WHERE time BETWEEN ?1 AND ?2",
                        params![from_time, to_time],
                        |row| {
                            Ok((
                                Item::new_sales(row)?,
                                //method
                                Payment::try_from(row.get::<usize, String>(4)?).unwrap_or_default(),
                            ))
                        },
                        Vec<(Item, Payment)>
                    ),
                }
            }

            Self::Manager => Tab::Manager(sql!(
                "SELECT name, price, available, category FROM menu \
                    WHERE special = 0 
                    ORDER BY 
                        CASE category 
                            WHEN 'alcohol' THEN 1
                            WHEN 'drink' THEN 2
                            WHEN 'food' THEN 3
                            WHEN 'other' THEN 4
                            ELSE 5
                        END,
                        name DESC",
                params![],
                Item::new_stock,
                Vec<_>
            )),

            Self::Info => Tab::Info(crate::config::update()?),
        }))
    }
}

#[derive(Clone)]
pub struct Sideffect(futures::future::Shared<BoxFuture<'static, Result<Message>>>);

impl Sideffect {
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Message>> + Send + 'static,
    {
        Self(f().boxed().shared())
    }
}

impl IntoFuture for Sideffect {
    type Output = Result<Message>;
    type IntoFuture = futures::future::Shared<BoxFuture<'static, Result<Message>>>;

    fn into_future(self) -> Self::IntoFuture {
        self.0
    }
}

impl std::fmt::Debug for Sideffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sideffect(_)")
    }
}

#[derive(Clone, Debug, Default)]
pub enum Message {
    #[default]
    None,
    SwapTab(TabId),
    LoadTab(Tab),
    CloseModal,
    OpenModal {
        title: &'static str,
        content: String,
    },
    Sideffect(Sideffect),
}

impl From<()> for Message {
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl<T, E> From<Result<T, E>> for Message
where
    T: Into<Message>,
    E: Into<Error>,
{
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(t) => t.into(),
            Err(e) => Self::OpenModal {
                title: "Error",
                content: format!("{:#?}", e.into()),
            },
        }
    }
}

//Allow data to trickle back down, only really used in sales
impl From<Tab> for Message {
    fn from(value: Tab) -> Self {
        Self::LoadTab(value)
    }
}

impl From<Sideffect> for Message {
    fn from(value: Sideffect) -> Self {
        Self::Sideffect(value)
    }
}
