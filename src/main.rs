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
        styles::{DEF_PADDING, DEF_TEXT, SMALL_TEXT},
    },
    iced::{window, Application, Column, Command, Element, Font, Length, Settings, Text},
    iced_aw::{
        modal::{self, Modal},
        Card, TabLabel, Tabs,
    },
    rusqlite::Connection,
    std::sync::{Arc, Mutex},
};

pub mod config;
pub mod error;
pub mod icons;
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
    con: Arc<Mutex<Connection>>,
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
                con: match config::init_db() {
                    Ok(con) => Arc::new(Mutex::new(con)),
                    Err(e) => panic!("{:#?}", e),
                },
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
            Message::DB(f) => match f(self.con.clone()) {
                Ok(Message::Menu(m)) => self.menu.update(m),
                Ok(Message::Transactions(m)) => self.transactions.update(m),
                Ok(Message::Manager(m)) => self.manager.update(m),
                Ok(Message::Sales(m)) => self.sales.update(m),
                Err(e) => command_now!(Message::Error(e)),
                _ => Command::none(),
            },
            Message::CloseModal => {
                *self.err.inner_mut() = None;
                Command::none()
            }
            Message::Error(e) => {
                *self.err.inner_mut() = Some(e);
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
                Tabs::new(self.tab, Message::SwapTab)
                    .icon_font(icons::ICON_FONT)
                    .height(Length::Shrink)
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
