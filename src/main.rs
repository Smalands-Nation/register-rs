use {
    crate::{
        icons::Icon,
        screens::{Message, Tab, TabId},
        theme::{DEF_PADDING, DEF_TEXT, TabStyle},
        widgets::SMALL_TEXT,
    },
    chrono::Local,
    iced::{
        Application, Command, Element, Font, Length, Pixels, Settings, Size, font,
        widget::{Container, Text},
        window,
    },
    iced_aw::{Card, Modal, TabLabel, Tabs},
};

pub mod config;
pub mod error;
pub mod icons;
pub mod item;
pub mod receipt;
pub mod screens;
pub mod theme;
pub mod widgets;

#[macro_export]
macro_rules! command {
    ($msg:expr) => {
        Command::perform(async move { $msg }, $crate::screens::Message::from)
    };
}

pub const FONT: Font = Font::with_name("IBM Plex Mono");

pub fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            min_size: Some(Size {
                width: 1360.0,
                height: 600.0,
            }),
            ..window::Settings::default()
        },
        default_font: FONT,
        default_text_size: Pixels(DEF_TEXT),
        ..Settings::default()
    })
}

struct App {
    modal: Option<(&'static str, String)>,
    tab: Tab,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::Theme;

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                modal: None,
                tab: Tab::Menu(Vec::new()),
            },
            Command::batch([
                command!(config::init_db()),
                command!(config::set_receipt_path()),
                font::load(include_bytes!("../resources/IBMPlexMono-Regular.ttf").as_slice())
                    .map(Message::from),
                font::load(include_bytes!("../resources/google-fonts-icons.ttf").as_slice())
                    .map(Message::from),
                font::load(iced_aw::core::icons::BOOTSTRAP_FONT_BYTES).map(Message::from),
                command!(TabId::Menu.load().await),
            ]),
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
                    println!("Message::Error({content})");
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
        Modal::new(
            Container::new(
                Tabs::new(Message::SwapTab)
                    .tab_bar_style(TabStyle.into())
                    .text_size(DEF_TEXT)
                    .icon_size(DEF_TEXT)
                    .icon_font(icons::ICON_FONT)
                    .height(Length::Shrink)
                    .push(
                        TabId::Menu,
                        TabLabel::IconText(Icon::Menu.into(), String::from("Meny")),
                        self.tab.as_menu(),
                    )
                    .push(
                        TabId::Transactions,
                        TabLabel::IconText(Icon::Receipt.into(), String::from("Kvitton")),
                        self.tab.as_transactions(),
                    )
                    .push(
                        TabId::Sales {
                            from: Local::now().date_naive(),
                            to: Local::now().date_naive(),
                        },
                        TabLabel::IconText(Icon::Money.into(), String::from("Försäljning")),
                        self.tab.as_sales(),
                    )
                    .push(
                        TabId::Manager,
                        TabLabel::IconText(Icon::Settings.into(), String::from("Hantera")),
                        self.tab.as_manager(),
                    )
                    .push(
                        TabId::Info,
                        TabLabel::IconText(Icon::Info.into(), String::from("Systeminfo")),
                        self.tab.as_info(),
                    )
                    .set_active_tab(&self.tab.id()),
            )
            .padding(2),
            self.modal.clone().map(move |(title, content)| {
                Card::new(Text::new(title), SMALL_TEXT::new(content))
                    .max_width(650.0)
                    .padding(DEF_PADDING.into())
                    .on_close(Message::CloseModal)
            }),
        )
        .backdrop(Message::CloseModal)
        .into()
    }
}
