use {
    super::{db, Screen},
    crate::{
        icons::Icon,
        styles::{BIG_TEXT, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{Grid, NumberInput, SquareButton, TextInput},
    },
    iced::{
        button::{self, Button},
        scrollable::{self, Scrollable},
        Align, Column, Command, Container, Element, Length, Row, Rule, Space, Text,
    },
    iced_aw::{
        modal::{self, Modal},
        Card,
    },
    indexmap::IndexMap,
    rusqlite::params,
    std::future,
};

pub mod item;
pub use item::Item;

#[derive(Debug, Clone)]
pub enum Mode {
    New,
    Update(String),
}

pub struct Manager {
    locked: bool,
    login_modal: modal::State<(TextInput, button::State)>,
    menu: IndexMap<String, Item>,
    mode: Mode,
    name: TextInput,
    price: NumberInput<u32>,
    cancel: button::State,
    save: button::State,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh(bool),
    ToggleItem(String, bool),
    LoadMenu(Vec<Item>),
    EditItem(Item),
    UpdateName(String),
    UpdatePrice(Option<u32>),
    Cancel,
    Save,
    OpenLogin,
    CloseLogin,
    UpdatePassword(String),
    Login,
    Lock,
    Unlock,
}

impl Screen for Manager {
    type ExMessage = super::Message;
    type InMessage = Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                locked: true,
                login_modal: modal::State::new((TextInput::new(), button::State::new())),
                menu: IndexMap::new(),
                mode: Mode::New,
                name: TextInput::new(),
                price: NumberInput::new(),
                cancel: button::State::new(),
                save: button::State::new(),
                scroll: scrollable::State::new(),
            },
            future::ready(Message::Refresh(true).into()).into(),
        )
    }

    fn update(&mut self, msg: Self::InMessage) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh(lock) => {
                return Command::batch(
                    [
                        db(|con| {
                            Ok(Message::LoadMenu(
                        con.lock()
                            .unwrap()
                            .prepare("SELECT name, price, available FROM menu WHERE special = 0 ORDER BY name DESC")?
                            .query_map(params![], |row| {
                                Ok(Item::new(
                                    row.get::<usize, String>(0)?.as_str(),
                                    row.get(1)?,
                                    row.get(2)?,
                                ))
                            })?
                            .map(|item| item.unwrap())
                            .collect(),
                    )
                    .into())
                        }),
                        Command::perform(future::ready(Message::Cancel), Self::ExMessage::from),
                        Command::perform(future::ready(Message::Lock), Self::ExMessage::from),
                    ].into_iter().take(if lock { 3 } else { 2 }),
                );
            }
            Message::ToggleItem(name, a) => match self.menu.get_mut(&name) {
                Some(i) => {
                    i.available = a;
                    let clone = i.clone();
                    return db(move |con| {
                        con.lock().unwrap().execute(
                            "UPDATE menu SET available=?1 WHERE name=?2",
                            //Non breaking space gang
                            params![clone.available, clone.name],
                        )?;
                        Ok(Message::Refresh(false).into())
                    });
                }
                None => (),
            },
            Message::LoadMenu(m) => {
                self.menu.clear();
                for item in m {
                    self.menu.insert(item.clone().name, item);
                }
            }
            Message::EditItem(i) => {
                self.mode = Mode::Update(i.name.clone());
                self.name.update(i.name);
                self.price.update(Some(i.price));
            }
            Message::UpdateName(s) => self.name.update(s),
            Message::UpdatePrice(n) => self.price.update(n),
            Message::Cancel => {
                self.mode = Mode::New;
                self.name.update(String::new());
                self.price.update(None);
            }
            Message::Save => {
                let name = self.name.value();
                let price = self.price.value().unwrap_or(0);
                if !name.is_empty() {
                    return match &self.mode {
                        Mode::New => db(move |con| {
                            con.lock().unwrap().execute(
                                "INSERT INTO menu (name, price, available) VALUES (?1, ?2, true)",
                                //Non breaking space gang
                                params![name.replace(" ", "\u{00A0}"), price],
                            )?;
                            Ok(Message::Refresh(false).into())
                        }),
                        Mode::Update(old_name) => {
                            let old_name = old_name.clone();
                            db(move |con| {
                                con.lock().unwrap().execute(
                                    "UPDATE menu SET name=?1, price=?2 WHERE name=?3",
                                    //Non breaking space gang
                                    params![name.replace(" ", "\u{00A0}"), price, old_name],
                                )?;
                                Ok(Message::Refresh(false).into())
                            })
                        }
                    };
                }
            }
            Message::OpenLogin => self.login_modal.show(true),
            Message::CloseLogin => {
                self.login_modal.show(false);
                self.login_modal.inner_mut().0.update(String::new());
            }
            Message::UpdatePassword(password) => self.login_modal.inner_mut().0.update(password),
            Message::Login => {
                let password = self.login_modal.inner_mut().0.value();
                return db(move |con| {
                    Ok(if con.lock().unwrap().query_row(
                        "SELECT * FROM password WHERE password = ?1",
                        params![password],
                        |row| Ok(!row.get::<usize, String>(0)?.is_empty()),
                    )? {
                        Message::Unlock
                    } else {
                        Message::Lock
                    }
                    .into())
                });
            }
            Message::Lock => {
                self.locked = true;
                return Command::perform(future::ready(Message::CloseLogin), Self::ExMessage::from);
            }
            Message::Unlock => {
                self.locked = false;
                return Command::perform(future::ready(Message::CloseLogin), Self::ExMessage::from);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(Modal::new(
            &mut self.login_modal,
            Row::with_children(vec![
                Scrollable::new(&mut self.scroll)
                    .push(
                        Grid::with_children(
                            self.menu.len() as u32 / 3,
                            3,
                            self.menu.iter_mut().map(|(_, i)| i.view()).collect(),
                        )
                        .width(Length::Fill)
                        .spacing(DEF_PADDING)
                        .padding(DEF_PADDING),
                    )
                    .width(Length::Fill)
                    .spacing(DEF_PADDING)
                    .padding(DEF_PADDING)
                    .into(),
                Rule::vertical(DEF_PADDING).into(),
                Column::with_children(vec![
                    Row::new()
                        .push(
                            Column::with_children(match &self.mode {
                                Mode::New => {
                                    vec![
                                        Text::new("Ny").size(BIG_TEXT).into(),
                                        Text::new(" ").into(),
                                    ]
                                }
                                Mode::Update(v) => vec![
                                    Text::new("Ändrar").size(BIG_TEXT).into(),
                                    Text::new(v).into(),
                                ],
                            })
                            .width(Length::Fill),
                        )
                        .push(
                            SquareButton::new(&mut self.cancel, Text::from(Icon::Cross))
                                .on_press(Message::Cancel),
                        )
                        .align_items(Align::Start)
                        .into(),
                    Space::with_height(Length::FillPortion(1)).into(),
                    Text::new("Namn").into(),
                    self.name
                        .build("", Message::UpdateName)
                        .padding(DEF_PADDING)
                        .into(),
                    Text::new("Pris (kr)").into(),
                    self.price
                        .build(1, 1000, Message::UpdatePrice)
                        .padding(DEF_PADDING)
                        .width(Length::Fill)
                        .into(),
                    Space::with_height(Length::FillPortion(5)).into(),
                    if !self.locked {
                        Button::new(&mut self.save, Text::new("Spara").size(BIG_TEXT))
                            .on_press(Message::Save)
                            .padding(DEF_PADDING)
                            .width(Length::Fill)
                            .into()
                    } else {
                        Button::new(
                            &mut self.save,
                            Row::with_children(vec![
                                Text::new("Spara").size(BIG_TEXT).into(),
                                Space::with_width(Length::Fill).into(),
                                Text::from(Icon::Lock).into(),
                            ]),
                        )
                        .on_press(Message::OpenLogin)
                        .padding(DEF_PADDING)
                        .width(Length::Fill)
                        .into()
                    },
                ])
                .width(Length::Units(RECEIPT_WIDTH))
                .spacing(DEF_PADDING)
                .padding(DEF_PADDING)
                .into(),
            ]),
            |(password, bttn)| {
                Card::new(
                    Text::new("Login krävs för att ändra i produkt"),
                    Column::with_children(vec![
                        Text::new("Lösendord").into(),
                        password
                            .build("", Message::UpdatePassword)
                            .password()
                            .padding(DEF_PADDING)
                            .into(),
                        Button::new(bttn, Text::new("Logga In"))
                            .on_press(Message::Login)
                            .into(),
                    ])
                    .padding(DEF_PADDING)
                    .spacing(DEF_PADDING),
                )
                .max_width(650)
                .on_close(Message::CloseLogin)
                .into()
            },
        ))
        .map(Self::ExMessage::Manager)
    }
}
