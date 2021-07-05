use {
    crate::{
        error::Error,
        icons::Icon,
        screens::{
            manager::{self, Manager},
            menu::{self, Menu},
            transactions::{self, Transactions},
            Message, Screen,
        },
        styles::{DEF_PADDING, DEF_TEXT, SMALL_TEXT},
    },
    iced::{
        window, Application, Clipboard, Column, Command, Element, Font, Length, Settings, Text,
    },
    iced_aw::{
        modal::{self, Modal},
        Card, TabLabel, Tabs,
    },
    rusqlite::Connection,
    std::sync::{Arc, Mutex},
};

pub mod error;
pub mod icons;
pub mod payment;
pub mod screens;
pub mod styles;
pub mod widgets;

pub const FONT: Font = Font::External {
    name: "IBM Plex Mono",
    bytes: include_bytes!("../resources/IBMPlexMono-Regular.ttf"),
};

//pub type Marc<T> = Arc<Mutex<T>>;

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
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut cmds = Vec::new();

        let (menu, mcmd) = Menu::new();
        cmds.push(mcmd);

        let (transactions, mcmd) = Transactions::new();
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
                transactions,
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
                    2 => self.manager.update(manager::Message::Refresh),
                    1 => self.transactions.update(transactions::Message::Refresh),
                    _ => self.menu.update(menu::Message::Refresh),
                }
            }
            Message::DB(f) => match f(self.con.clone()) {
                Ok(Message::Menu(m)) => self.menu.update(m),
                Ok(Message::Transactions(m)) => self.transactions.update(m),
                Ok(Message::Manager(m)) => self.manager.update(m),
                Err(e) => {
                    *self.err.inner_mut() = Some(e.into());
                    Command::none()
                }
                _ => Command::none(),
            },
            Message::CloseModal => {
                *self.err.inner_mut() = None;
                Command::none()
            }
            Message::Menu(msg) => self.menu.update(msg),
            Message::Transactions(msg) => self.transactions.update(msg),
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
                        TabLabel::IconText(Icon::Reciept.into(), String::from("Kvitton")),
                        self.transactions.view(),
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
