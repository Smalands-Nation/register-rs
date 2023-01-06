//TODO use new iced_lazy::components for custom widgets and such
//maybe screens can be moved to components too?
//TODO fix styling, might need to impl custom Theme to pass stylesheets
use {
    crate::{
        error::Error,
        icons::Icon,
        screens::{
            info::Info,
            manager::{self, Manager},
            menu::{self, Menu},
            sales::{self, Sales},
            transactions::{self, Transactions},
            Message, Screen,
        },
        styles::{BORDER_WIDTH, DEF_PADDING, DEF_TEXT},
        widgets::{column, SMALL_TEXT},
    },
    iced::{
        widget::{Container, Text},
        window, Application, Command, Font, Length, Settings,
    },
    iced_aw::{Card, Modal, TabLabel, Tabs},
    lazy_static::lazy_static,
    rusqlite::Connection,
    std::sync::Arc,
    tokio::sync::Mutex,
};

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

pub mod theme;

pub type Renderer = iced::Renderer<theme::Theme>;
pub type Element<'a, M> = iced::Element<'a, M, Renderer>;

#[macro_export]
macro_rules! command {
    ($msg:expr) => {
        Command::perform(async move { $msg }, $crate::screens::Message::from)
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
    err: Option<Error>,
    tab: usize,
    menu: Menu,
    transactions: Transactions,
    manager: Manager,
    sales: Sales,
    info: Info,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = theme::Theme;

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut cmds = vec![];

        let (menu, mcmd) = Menu::new();
        cmds.push(mcmd);

        let (transactions, mcmd) = Transactions::new();
        cmds.push(mcmd);

        let (manager, mcmd) = Manager::new();
        cmds.push(mcmd);

        let (sales, mcmd) = Sales::new();
        cmds.push(mcmd);

        let (info, mcmd) = Info::new();
        cmds.push(mcmd);

        (
            Self {
                err: None,
                tab: 0,
                menu,
                transactions,
                manager,
                sales,
                info,
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
                    //info doesn't refresh
                    3 => self.manager.update(manager::Message::Refresh(true)),
                    2 => self.sales.update(sales::Message::Refresh),
                    1 => self.transactions.update(transactions::Message::Refresh),
                    _ => self.menu.update(menu::Message::Refresh),
                }
            }
            //Message::DB(f) => f(self.con.clone()),
            Message::CloseModal => {
                self.err = None;
                Command::none()
            }
            Message::Error(e) => {
                println!("Message::Error({:#?})", e);
                self.err = Some(e);
                //TODO add logging here
                Command::none()
            }
            Message::Menu(msg) => self.menu.update(msg),
            Message::Transactions(msg) => self.transactions.update(msg),
            Message::Manager(msg) => self.manager.update(msg),
            Message::Sales(msg) => self.sales.update(msg),
            Message::Info(msg) => self.info.update(msg),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let err = self.err.clone();
        Modal::new(
            self.err.is_some(),
            column![
                #nopad
                Container::new(
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
                        )
                        .push(
                            TabLabel::IconText(Icon::Info.into(), String::from("Systeminfo")),
                            self.info.view(),
                        ),
                )
                .padding(BORDER_WIDTH as u16),
            ],
            move || {
                Card::new(
                    Text::new("Error"),
                    SMALL_TEXT::new(match &err {
                        Some(e) => format!("{:#?}", e),
                        None => String::new(),
                    }),
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
