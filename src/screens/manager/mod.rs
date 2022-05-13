use {
    super::{db, Screen},
    crate::{
        command_now,
        icons::Icon,
        query,
        styles::{BIG_TEXT, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{Grid, NumberInput, SquareButton},
    },
    iced::{
        pure::{
            widget::{Button, Column, Row, Rule, Scrollable, Text, TextInput},
            Pure, State,
        },
        Alignment, Command, Element, Length, Space,
    },
    iced_aw::{
        modal::{self, Modal},
        Card,
    },
    indexmap::IndexMap,
    rusqlite::params,
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
    login_modal: modal::State<(String, State)>,
    under_modal: State,
    menu: IndexMap<String, Item>,
    mode: Mode,
    name: String,
    price: NumberInput<u32>,
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
                login_modal: modal::State::new((String::new(), State::new())),
                under_modal: State::new(),
                menu: IndexMap::new(),
                mode: Mode::New,
                name: String::new(),
                price: NumberInput::new(),
            },
            command_now!(Message::Refresh(true).into()),
        )
    }

    fn update(&mut self, msg: Self::InMessage) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh(lock) => {
                return Command::batch(
                    [
                        query!(
                            "SELECT name, price, available FROM menu \
                            WHERE special = 0 ORDER BY name DESC",
                            row => Item::new(
                                row.get::<usize, String>(0)?.as_str(),
                                row.get(1)?,
                                row.get(2)?,
                            ),
                            Message::LoadMenu
                        ),
                        command_now!(Message::Cancel.into()),
                        command_now!(Message::Lock.into()),
                    ]
                    .into_iter()
                    .take(if lock { 3 } else { 2 }),
                );
            }
            Message::ToggleItem(name, a) => {
                if let Some(i) = self.menu.get_mut(&name) {
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
            }
            Message::LoadMenu(m) => {
                self.menu.clear();
                for item in m {
                    self.menu.insert(item.clone().name, item);
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
                self.login_modal.inner_mut().0.clear();
            }
            Message::UpdatePassword(password) => {
                self.login_modal.inner_mut().0 = password;
            }
            //No password in debug mode
            #[cfg(debug_assertions)]
            Message::Login => {
                return command_now!(Message::CloseLogin.into());
            }
            //Use env for password
            #[cfg(not(debug_assertions))]
            Message::Login => {
                let password_ok = self.login_modal.inner().0.as_str() == env!("SMALANDS_PASSWORD");
                return command_now!(if password_ok {
                    Message::Unlock
                } else {
                    Message::Lock
                }
                .into());
            }
            Message::Lock => {
                self.locked = true;
                return command_now!(Message::CloseLogin.into());
            }
            Message::Unlock => {
                self.locked = false;
                return command_now!(Message::CloseLogin.into());
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(Modal::new(
            &mut self.login_modal,
            Pure::new(
                &mut self.under_modal,
                Row::with_children(vec![
                    Scrollable::new(
                        Grid::with_children(
                            self.menu.len() as u32 / 3,
                            3,
                            self.menu.iter_mut().map(|(_, i)| i.view()).collect(),
                        )
                        .width(Length::Fill)
                        .spacing(DEF_PADDING)
                        .padding(DEF_PADDING),
                    )
                    //.scroller_width(Length::Fill) //NOTE not sure if correct field
                    //.spacing(DEF_PADDING)
                    //.padding(DEF_PADDING)
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
                                SquareButton::new(Text::from(Icon::Cross))
                                    .on_press(Message::Cancel),
                            )
                            .align_items(Alignment::Start)
                            .into(),
                        Space::with_height(Length::FillPortion(1)).into(),
                        Text::new("Namn").into(),
                        TextInput::new("", self.name.as_str(), Message::UpdateName)
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
                            Button::new(Text::new("Spara").size(BIG_TEXT))
                                .on_press(Message::Save)
                                .padding(DEF_PADDING)
                                .width(Length::Fill)
                                .into()
                        } else {
                            Button::new(Row::with_children(vec![
                                Text::new("Spara").size(BIG_TEXT).into(),
                                Space::with_width(Length::Fill).into(),
                                Text::from(Icon::Lock).into(),
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
            ),
            |(password, in_modal)| {
                Card::new(
                    Text::new("Login krävs för att ändra i produkt"),
                    Pure::new(
                        in_modal,
                        Column::with_children(vec![
                            Text::new("Lösendord").into(),
                            TextInput::new("", password, Message::UpdatePassword)
                                .password()
                                .padding(DEF_PADDING)
                                .into(),
                            Button::new(Text::new("Logga In"))
                                .on_press(Message::Login)
                                .into(),
                        ])
                        .padding(DEF_PADDING)
                        .spacing(DEF_PADDING),
                    ),
                )
                .max_width(650)
                .on_close(Message::CloseLogin)
                .into()
            },
        ))
        .map(Self::ExMessage::Manager)
    }
}
