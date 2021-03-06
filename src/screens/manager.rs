use {
    super::Screen,
    crate::{
        command,
        icons::Icon,
        item::{kind::Stock, Item},
        sql,
        styles::{DEF_PADDING, RECEIPT_WIDTH},
        widgets::{Grid, NumberInput, SquareButton, BIG_TEXT},
    },
    iced::{
        pure::{
            widget::{Button, Column, Row, Rule, Scrollable, Text, TextInput},
            Element,
        },
        Alignment, Command, Length, Space,
    },
    iced_aw::pure::{Card, Modal},
    rusqlite::params,
};

#[derive(Debug, Clone)]
pub enum Mode {
    New,
    Update(String),
}

pub struct Manager {
    locked: bool,
    login_modal: bool,
    password: String,
    menu: Vec<Item<Stock>>,
    mode: Mode,
    name: String,
    price: NumberInput<i32>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh(bool),
    ToggleItem(usize, bool),
    LoadMenu(Vec<Item<Stock>>),
    EditItem(Item<Stock>),
    UpdateName(String),
    UpdatePrice(Option<i32>),
    Cancel,
    Save,
    OpenLogin,
    CloseLogin,
    UpdatePassword(String),
    Login,
    Lock,
    Unlock,
}

impl<'a, 's> Screen for Manager
where
    Self: 's,
    's: 'a,
{
    type ExMessage = super::Message;
    type InMessage = Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                locked: true,
                login_modal: false,
                password: String::new(),
                menu: Vec::new(),
                mode: Mode::New,
                name: String::new(),
                price: NumberInput::new(),
            },
            command!(Message::Refresh(true)),
        )
    }

    fn update(&mut self, msg: Self::InMessage) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh(lock) => {
                return Command::batch(
                    [
                        sql!(
                            "SELECT name, price, available FROM menu \
                            WHERE special = 0 ORDER BY name DESC",
                            params![],
                            |row| {
                                Ok(Item {
                                    name: row.get::<usize, String>(0)?,
                                    price: row.get(1)?,
                                    kind: Stock {
                                        idx: 0,
                                        available: row.get("available")?,
                                    },
                                })
                            },
                            Vec<_>,
                            Message::LoadMenu
                        ),
                        command!(Message::Cancel),
                        command!(Message::Lock),
                    ]
                    .into_iter()
                    .take(if lock { 3 } else { 2 }),
                );
            }
            Message::ToggleItem(i, a) => {
                if let Some(i) = self.menu.get_mut(i) {
                    i.in_stock(a);
                    let clone = i.clone();
                    return sql!(
                        "UPDATE menu SET available=?1 WHERE name=?2",
                        //Non breaking space gang
                        params![a, clone.name.replace(' ', "\u{00A0}")],
                        Message::Refresh(false)
                    );
                }
            }
            Message::LoadMenu(m) => {
                self.menu.clear();
                for (i, item) in m.into_iter().enumerate() {
                    self.menu.push(item.with_index(i));
                }
            }
            Message::EditItem(i) => {
                self.mode = Mode::Update(i.name.clone());
                self.name = i.name;
                self.price.update(Some(i.price));
            }
            Message::UpdateName(s) => self.name = s,
            Message::UpdatePrice(n) => self.price.update(n),
            Message::Cancel => {
                self.mode = Mode::New;
                self.name.clear();
                self.price.update(None);
            }
            Message::Save => {
                let name = self.name.clone();
                let price = self.price.value().unwrap_or(0);
                if !name.is_empty() {
                    return match &self.mode {
                        Mode::New => sql!(
                            "INSERT INTO menu (name, price, available) VALUES (?1, ?2, true)",
                            //Non breaking space gang
                            params![name.replace(' ', "\u{00A0}"), price],
                            Message::Refresh(false)
                        ),
                        Mode::Update(old_name) => {
                            let old_name = old_name.clone();
                            sql!(
                                "UPDATE menu SET name=?1, price=?2 WHERE name=?3",
                                //Non breaking space gang
                                params![name.replace(' ', "\u{00A0}"), price, old_name],
                                Message::Refresh(false)
                            )
                        }
                    };
                }
            }
            Message::OpenLogin => self.login_modal = true,
            Message::CloseLogin => self.login_modal = false,
            Message::UpdatePassword(password) => {
                self.password = password;
            }
            //No password in debug mode
            #[cfg(debug_assertions)]
            Message::Login => {
                return command!(Message::Unlock);
            }
            //Use env for password
            #[cfg(not(debug_assertions))]
            Message::Login => {
                let password_ok = self.password == env!("SMALANDS_PASSWORD");
                return command!(if password_ok {
                    Message::Unlock
                } else {
                    Message::Lock
                });
            }
            Message::Lock => {
                self.locked = true;
                return command!(Message::CloseLogin);
            }
            Message::Unlock => {
                self.locked = false;
                return command!(Message::CloseLogin);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(Modal::new(
            self.login_modal,
            Row::with_children(vec![
                Scrollable::new(
                    Grid::with_children(
                        self.menu.len() as u32 / 3,
                        3,
                        self.menu
                            .iter()
                            .map(|item| item.as_widget().on_press(Message::EditItem).into())
                            .collect(),
                    )
                    .width(Length::Fill)
                    .spacing(DEF_PADDING)
                    .padding(DEF_PADDING),
                )
                .into(),
                Rule::vertical(DEF_PADDING).into(),
                Column::with_children(vec![
                    Row::new()
                        .push(
                            Column::with_children(match &self.mode {
                                Mode::New => {
                                    vec![BIG_TEXT::new("Ny").into(), Text::new(" ").into()]
                                }
                                Mode::Update(v) => {
                                    vec![BIG_TEXT::new("??ndrar").into(), Text::new(v).into()]
                                }
                            })
                            .width(Length::Fill),
                        )
                        .push(SquareButton::new(Icon::Cross).on_press(Message::Cancel))
                        .align_items(Alignment::Start)
                        .into(),
                    Space::with_height(Length::FillPortion(1)).into(),
                    Text::new("Namn").into(),
                    TextInput::new("", self.name.as_str(), Message::UpdateName)
                        .padding(DEF_PADDING)
                        .into(),
                    Text::new("Pris (kr)").into(),
                    self.price
                        .build(1..=1000, Message::UpdatePrice)
                        .padding(DEF_PADDING)
                        .width(Length::Fill)
                        .into(),
                    Space::with_height(Length::FillPortion(5)).into(),
                    if !self.locked {
                        Button::new(BIG_TEXT::new("Spara"))
                            .on_press(Message::Save)
                            .padding(DEF_PADDING)
                            .width(Length::Fill)
                            .into()
                    } else {
                        Button::new(Row::with_children(vec![
                            BIG_TEXT::new("Spara").into(),
                            Space::with_width(Length::Fill).into(),
                            BIG_TEXT::new(Icon::Lock).into(),
                        ]))
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
            || {
                Card::new(
                    Text::new("Login kr??vs f??r att ??ndra i produkt"),
                    Column::with_children(vec![
                        Text::new("L??sendord").into(),
                        TextInput::new("", &self.password, Message::UpdatePassword)
                            .password()
                            .padding(DEF_PADDING)
                            .on_submit(Message::Login)
                            .into(),
                        Button::new(Text::new("Logga In"))
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
