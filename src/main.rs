use {
    crate::{
        error::Error,
        icons::Icon,
        screens::{manager, menu, Manager, Menu, Message, Screen},
    },
    iced::{
        window, Application, Clipboard, Column, Command, Element, Font, Length, Settings, Text,
    },
    iced_aw::{modal, Card, Modal, TabLabel, Tabs},
    rusqlite::{params, Connection},
    std::sync::{Arc, Mutex},
};

pub mod error;
pub mod icons;
pub mod screens;
pub mod styles;
pub mod widgets;

pub const BIG_TEXT: u16 = 45;
pub const DEF_TEXT: u16 = 35;
pub const SMALL_TEXT: u16 = 20;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const FONT: Font = Font::External {
    name: "IBM Plex Mono",
    bytes: include_bytes!("../resources/IBMPlexMono-Regular.ttf"),
};

pub type Marc<T> = Arc<Mutex<T>>;

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
    con: Marc<Connection>,
    err: modal::State<Option<Error>>,
    tab: usize,
    menu: Menu,
    manager: Manager,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut cmds = Vec::new();

        let (menu, mcmd) = Menu::new();
        cmds.push(mcmd);

        let (manager, mcmd) = Manager::new();
        cmds.push(mcmd);

        (
            Self {
                con: match Connection::open("./smaland.db") {
                    Ok(con) => Arc::new(Mutex::new(con)),
                    Err(e) => panic!("{}", e),
                },
                err: modal::State::new(None),
                tab: 0,
                menu,
                manager,
            },
            Command::batch(cmds),
        )
    }

    fn title(&self) -> String {
        String::from("Kassa")
    }

    fn update(&mut self, msg: Self::Message, _clip: &mut Clipboard) -> Command<Self::Message> {
        match msg {
            Message::SwapTab(n) => {
                self.tab = n;
                match n {
                    1 => self.manager.update(manager::Message::Refresh),
                    _ => self.menu.update(menu::Message::Refresh),
                }
            }
            Message::ReadDB(f) => match f(self.con.clone()) {
                Ok(Message::Menu(m)) => self.menu.update(m),
                Ok(Message::Manager(m)) => self.manager.update(m),
                Err(e) => {
                    *self.err.inner_mut() = Some(e.into());
                    Command::none()
                }
                _ => Command::none(),
            },
            Message::WriteDB(q) => match self.con.lock().unwrap().execute(&q, params![]) {
                Ok(_) => Command::none(),
                Err(e) => {
                    *self.err.inner_mut() = Some(e.into());
                    Command::none()
                }
            },
            Message::CloseModal => {
                *self.err.inner_mut() = None;
                Command::none()
            }
            Message::Menu(msg) => self.menu.update(msg),
            Message::Manager(msg) => self.manager.update(msg),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        if let Some(_) = self.err.inner() {
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
