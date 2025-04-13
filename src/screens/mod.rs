pub mod info;
//pub mod manager;
pub mod menu;
pub mod sales;
pub mod transactions;

use {
    crate::error::{Error, Result},
    backend::{items::Item, receipts::Receipt, summary::Summary},
    chrono::{DateTime, Local, NaiveDate},
    futures::{future::BoxFuture, FutureExt},
    iced::Element,
    indexmap::IndexMap,
    std::future::{Future, IntoFuture},
};

use {
    info::Info,
    //manager::Manager,
    menu::Menu,
    sales::Sales,
    transactions::Transactions,
};

#[derive(Clone, Debug)]
pub enum Tab {
    Menu(Vec<Item>),
    Transactions(IndexMap<DateTime<Local>, Receipt>),
    Sales(Summary),
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
        if let Self::Sales(summary) = self {
            Sales::new(summary.clone()).into()
        } else {
            iced::widget::Text::new("Empty").into()
        }
    }

    pub fn as_manager(&self) -> Element<Message> {
        if let Self::Manager(menu) = self {
            //Manager::new(menu.clone()).into()
            todo!()
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
            Self::Sales(summary) => TabId::Sales {
                from: summary.from().date_naive(),
                to: summary.to().date_naive(),
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
    Sales { from: NaiveDate, to: NaiveDate },
    Manager,
    Info,
}

impl PartialEq for TabId {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Menu, Self::Menu)
                | (Self::Transactions, Self::Transactions)
                | (Self::Sales { .. }, Self::Sales { .. })
                | (Self::Manager, Self::Manager)
                | (Self::Info, Self::Info)
        )
    }
}

impl TabId {
    pub async fn load(self) -> Result<Message> {
        Ok(Message::LoadTab(match self {
            Self::Menu => Tab::Menu(Item::get_all_available().await?),

            Self::Transactions => Tab::Transactions(Receipt::get_recents().await?),

            Self::Sales { from, to } => {
                let from_time = from
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .single()
                    .unwrap();
                let to_time = to
                    .and_hms_opt(23, 59, 59)
                    .unwrap()
                    .and_local_timezone(Local)
                    .single()
                    .unwrap();
                Tab::Sales(Summary::get_sales_summary(from_time, to_time).await?)
            }

            //Self::Manager => Tab::Manager(sql!(
            //    "SELECT name, price, available, category FROM menu \
            //        WHERE special = 0
            //        ORDER BY
            //            CASE category
            //                WHEN 'alcohol' THEN 1
            //                WHEN 'drink' THEN 2
            //                WHEN 'food' THEN 3
            //                WHEN 'other' THEN 4
            //                ELSE 5
            //            END,
            //            name DESC",
            //    params![],
            //    Item::new_stock,
            //    Vec<_>
            //)),
            Self::Info => Tab::Info(crate::config::update()?),
            _ => todo!(),
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
