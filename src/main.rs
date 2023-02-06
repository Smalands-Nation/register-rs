use {
    crate::{
        icons::Icon,
        screens::{Message, Tab},
        theme::{BORDER_WIDTH, DEF_PADDING, DEF_TEXT},
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
pub mod screens;
pub mod theme;
pub mod widgets;

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
    modal: Option<(&'static str, String)>,
    tab: Tab,
    //menu: Menu,
    //transactions: Transactions,
    //manager: Manager,
    //sales: Sales,
    //info: Info,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = theme::Theme;

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        //let mut cmds = vec![];

        //let (menu, mcmd) = Menu::new();
        //cmds.push(mcmd);

        //let (transactions, mcmd) = Transactions::new();
        //cmds.push(mcmd);

        //let (manager, mcmd) = Manager::new();
        //cmds.push(mcmd);

        //let (sales, mcmd) = Sales::new();
        //cmds.push(mcmd);

        //let (info, mcmd) = Info::new();
        //cmds.push(mcmd);

        (
            Self {
                modal: None,
                tab: Tab::Menu(Vec::new()),
                //menu,
                //transactions,
                //manager,
                //sales,
                //info,
            },
            command!(Tab::Menu(vec![]).load().await),
        )
    }

    fn title(&self) -> String {
        String::from("Kassa")
    }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::None => Command::none(),
            Message::SwapTab(tab) => command!(tab.load().await),
            Message::LoadTab(tab) => {
                self.tab = tab;
                Command::none()
            }
            Message::CloseModal => {
                self.modal = None;
                Command::none()
            }
            Message::OpenModal { title, content } => {
                if title == "Error" {
                    //TODO structured logging??
                    println!("Message::Error({content:#?})");
                }
                self.modal = Some((title, content));
                Command::none()
            }
            Message::Sideffect(f) => command! {
                Message::from(f.await)
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let modal = self.modal.clone();
        Modal::new(
            self.modal.is_some(),
            column![
                #nopad
                Container::new(
                    Tabs::new((&self.tab).into(), |n| Message::SwapTab(Tab::from(n)))
                        .icon_font(icons::ICON_FONT)
                        .height(Length::Shrink)
                        .push(
                            TabLabel::IconText(Icon::Menu.into(), String::from("Meny")),
                            self.tab.as_menu()
                        )
                        .push(
                            TabLabel::IconText(Icon::Receipt.into(), String::from("Kvitton")),
                            self.tab.as_transactions(),
                        )
                        .push(
                            TabLabel::IconText(Icon::Money.into(), String::from("Försäljning")),
                            self.tab.as_sales(),
                        )
                        .push(
                            TabLabel::IconText(Icon::Settings.into(), String::from("Hantera")),
                            self.tab.as_manager(),
                        )
                        .push(
                            TabLabel::IconText(Icon::Info.into(), String::from("Systeminfo")),
                            self.tab.as_info(),
                        ),
                )
                .padding(BORDER_WIDTH as u16),
            ],
            move || {
                let (title, content) = modal.clone().unwrap_or_default();
                Card::new(Text::new(title), SMALL_TEXT::new(content))
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
