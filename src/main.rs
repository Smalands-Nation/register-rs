use {
    crate::{
        error::{Error, Result},
        icons::Icon,
        screens::{Message, Screen},
    },
    calc::Calc,
    grid::Grid,
    iced::{
        button, scrollable, window, Application, Button, Checkbox, Clipboard, Column, Command,
        Container, Element, Font, HorizontalAlignment, Length, Row, Rule, Scrollable, Settings,
        Space, Text,
    },
    iced_aw::{TabBar, TabLabel},
    rusqlite::Connection,
    std::collections::HashMap,
};

pub mod calc;
pub mod error;
pub mod grid;
pub mod helper;
pub mod icons;
pub mod screens;

pub const BIG_TEXT: u16 = 45;
pub const DEF_TEXT: u16 = 35;
pub const SMALL_TEXT: u16 = 20;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const FONT: Font = Font::External {
    name: "IBM Plex Mono",
    bytes: include_bytes!("../resources/IBMPlexMono-Regular.ttf"),
};

pub fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            min_size: Some((1300, 600)),
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
    con: Option<Connection>,
    tab: usize,
    err: Option<Error>,
    screen: Screen,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            match Connection::open("../smaland.db") {
                Ok(con) => Self {
                    con: Some(con),
                    tab: 0,
                    err: None,
                    screen: screens::Menu::new(Vec::new()).into(),
                },
                Err(e) => Self {
                    con: None,
                    tab: 0,
                    err: Some(e.into()),
                    screen: screens::Menu::new(Vec::new()).into(),
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Kassa")
    }

    fn update(&mut self, msg: Self::Message, _clip: &mut Clipboard) -> Command<Self::Message> {
        match (&mut self.screen, msg) {
            (_, Message::SwapTab(n)) => {
                self.tab = n;
                match n {
                    0 => self.screen = screens::Menu::new(Vec::new()).into(),
                    _ => (),
                }
                Command::none()
            }
            (Screen::Menu(s), Message::Menu(msg)) => s.update(msg).map(Message::Menu),
            _ => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .push(
                TabBar::new(self.tab, Message::SwapTab)
                    .icon_font(icons::ICON_FONT)
                    .push(TabLabel::IconText(Icon::Menu.into(), String::from("Meny")))
                    .push(TabLabel::IconText(
                        Icon::Settings.into(),
                        String::from("Hantera"),
                    )),
            )
            .push(self.screen.view())
            .into()
    }
}
