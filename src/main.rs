use {
    crate::{
        error::Error,
        icons::Icon,
        screens::{
            manager::{self, Manager},
            menu::{self, Menu},
            sales::{self, Sales},
            transactions::{self, Transactions},
            Message, Screen,
        },
        styles::{BORDER_WIDTH, DEF_PADDING, DEF_TEXT, SMALL_TEXT, TABS},
    },
    iced::{
        window, Application, Column, Command, Container, Element, Font, Length, Settings, Text,
    },
    iced_aw::{
        modal::{self, Modal},
        Card, TabLabel, Tabs,
    },
    lazy_static::lazy_static,
    rusqlite::Connection,
    std::sync::Arc,
    tokio::sync::Mutex,
};

//TODO use iced_aw::pure and remove uses of pure
//TODO use iced_aw::Grid (needs pure)

//TODO use command_now more consistently

pub mod config;
pub mod error;
pub mod icons;
pub mod item;
pub mod payment;
pub mod print;
pub mod receipt;
#[allow(clippy::new_without_default)]
pub mod screens;
pub mod styles;
#[allow(clippy::new_ret_no_self, clippy::new_without_default)]
pub mod widgets;

#[macro_export]
macro_rules! command_now {
    ($msg:expr) => {
        Command::perform(async move { $msg }, |m| m)
    };
}

pub const FONT: Font = Font::External {
    name: "IBM Plex Mono",
    bytes: include_bytes!("../resources/IBMPlexMono-Regular.ttf"),
};

lazy_static! {
    pub static ref DB: Arc<Mutex<Connection>> =
        Arc::new(Mutex::new(config::init_db().expect("Fatal db error")));
}

pub fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            min_size: Some((1360, 600)),
            ..window::Settings::default()
        },
        default_font: match FONT {
            Font::External { bytes, .. } => Some(bytes),
            _ => None,
        },
        default_text_size: DEF_TEXT,
        ..Settings::default()
    })
}

struct App {
    err: modal::State<Option<Error>>,
    tab: usize,
    menu: Menu,
    transactions: Transactions,
    manager: Manager,
    sales: Sales,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut cmds = vec![command_now!(match config::update() {
            Ok(_) => Self::Message::None,
            Err(e) => Self::Message::Error(e),
        })];

        let (menu, mcmd) = Menu::new();
        cmds.push(mcmd);

        let (transactions, mcmd) = Transactions::new();
        cmds.push(mcmd);

        let (manager, mcmd) = Manager::new();
        cmds.push(mcmd);

        let (sales, mcmd) = Sales::new();
        cmds.push(mcmd);

        (
            Self {
                err: modal::State::new(None),
                tab: 0,
                menu,
                transactions,
                manager,
                sales,
            },
            Command::batch(cmds),
        )
    }

    fn title(&self) -> String {
        String::from("Kassa")
    }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::None => Command::none(),
            Message::SwapTab(n) => {
                self.tab = n;
                match n {
                    3 => self.manager.update(manager::Message::Refresh(true)),
                    2 => self.sales.update(sales::Message::Refresh),
                    1 => self.transactions.update(transactions::Message::Refresh),
                    _ => self.menu.update(menu::Message::Refresh),
                }
            }
            //Message::DB(f) => f(self.con.clone()),
            Message::CloseModal => {
                *self.err.inner_mut() = None;
                Command::none()
            }
            Message::Error(e) => {
                *self.err.inner_mut() = Some(e);
                //TODO add logging here
                Command::none()
            }
            Message::Menu(msg) => self.menu.update(msg),
            Message::Transactions(msg) => self.transactions.update(msg),
            Message::Manager(msg) => self.manager.update(msg),
            Message::Sales(msg) => self.sales.update(msg),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        if self.err.inner().is_some() {
            self.err.show(true);
        } else {
            self.err.show(false);
        }

        Modal::new(
            &mut self.err,
            Column::new().push(
                Container::new(
                    Tabs::new(self.tab, Message::SwapTab)
                        .icon_font(icons::ICON_FONT)
                        .height(Length::Shrink)
                        .tab_bar_style(TABS)
                        .push(
                            TabLabel::IconText(Icon::Menu.into(), String::from("Meny")),
                            self.menu.view(),
                        )
                        .push(
                            TabLabel::IconText(Icon::Receipt.into(), String::from("Kvitton")),
                            self.transactions.view(),
                        )
                        .push(
                            TabLabel::IconText(Icon::Money.into(), String::from("Försäljning")),
                            self.sales.view(),
                        )
                        .push(
                            TabLabel::IconText(Icon::Settings.into(), String::from("Hantera")),
                            self.manager.view(),
                        ),
                )
                .style(TABS)
                .padding(BORDER_WIDTH as u16),
            ),
            |state| {
                Card::new(
                    Text::new("Error"),
                    Text::new(match state {
                        Some(e) => format!("{:#?}", e),
                        None => String::new(),
                    })
                    .size(SMALL_TEXT),
                )
                .max_width(650)
                .padding(DEF_PADDING.into())
                .on_close(Message::CloseModal)
                .into()
            },
        )
        .backdrop(Message::CloseModal)
        .into()
    }
}
