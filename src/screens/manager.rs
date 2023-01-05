use {
    super::Screen,
    crate::{
        command,
        icons::Icon,
        item::{kind::Stock, Category, Item},
        sql,
        styles::{DEF_PADDING, RECEIPT_WIDTH},
        widgets::{column, row, Grid, NumberInput, SquareButton, BIG_TEXT},
    },
    iced::{
        widget::{Button, PickList, Rule, Scrollable, Space, Text, TextInput},
        Alignment, Command, Element, Length,
    },
    iced_aw::{Card, Modal},
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
    category: Option<Category>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh(bool),
    ToggleItem(usize, bool),
    LoadMenu(Vec<Item<Stock>>),
    EditItem(Item<Stock>),
    UpdateName(String),
    UpdatePrice(Option<i32>),
    UpdateCategory(Category),
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
                category: None,
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
                            "SELECT name, price, available, category FROM menu \
                            WHERE special = 0 
                            ORDER BY 
                                CASE category 
                                    WHEN 'alcohol' THEN 1
                                    WHEN 'drink' THEN 2
                                    WHEN 'food' THEN 3
                                    WHEN 'other' THEN 4
                                    ELSE 5
                                END,
                                name DESC",
                            params![],
                            Item::new_stock,
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
                self.category = Some(i.category);
            }
            Message::UpdateName(s) => self.name = s,
            Message::UpdatePrice(n) => self.price.update(n),
            Message::UpdateCategory(c) => self.category = Some(c),
            Message::Cancel => {
                self.mode = Mode::New;
                self.name.clear();
                self.price.update(None);
            }
            Message::Save => {
                let name = self.name.clone();
                let price = self.price.value().unwrap_or(0);
                let category = self.category;
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
                                "UPDATE menu SET name=?1, price=?2, category=?3 WHERE name=?4",
                                //Non breaking space gang
                                params![name.replace(' ', "\u{00A0}"), price, category, old_name],
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
            row![
                #nopad
                Scrollable::new(
                    Grid::with_children(
                        self.menu.len() as u32 / 3,
                        3,
                        self.menu
                            .iter()
                            .map(|item| item.on_press(Message::EditItem).into())
                            .collect(),
                    )
                    .width(Length::Fill)
                    .spacing(DEF_PADDING)
                    .padding(DEF_PADDING),
                ),
                Rule::vertical(DEF_PADDING),
                column![
                    row![
                        #nopad
                        BIG_TEXT::new(match &self.mode {
                            Mode::New => String::from("Ny"),
                            Mode::Update(v) => {
                                format!("Ändrar {}", v)
                            }
                        }),
                        Space::with_width(Length::Fill),
                        SquareButton::icon(Icon::Cross).on_press(Message::Cancel),
                    ]
                    .align_items(Alignment::Center),
                    Space::with_height(Length::FillPortion(1)),
                    Text::new("Namn"),
                    TextInput::new("", self.name.as_str(), Message::UpdateName)
                        .padding(DEF_PADDING),
                    Text::new("Pris (kr)"),
                    self.price
                        .build(1..=1000, Message::UpdatePrice)
                        .padding(DEF_PADDING)
                        .width(Length::Fill),
                        Text::new("Typ"),
                    PickList::new(&Category::ALL[..], self.category, Message::UpdateCategory),
                    Space::with_height(Length::FillPortion(5)),
                    if !self.locked {
                        Button::new(BIG_TEXT::new("Spara"))
                            .on_press(Message::Save)
                            .padding(DEF_PADDING)
                            .width(Length::Fill)
                    } else {
                        Button::new(row![
                            #nopad
                            BIG_TEXT::new("Spara"),
                            Space::with_width(Length::Fill),
                            Icon::Lock,
                        ])
                        .on_press(Message::OpenLogin)
                        .padding(DEF_PADDING)
                        .width(Length::Fill)
                    },
                ]
                .width(Length::Units(RECEIPT_WIDTH)),
            ],
            || {
                Card::new(
                    Text::new("Login krävs för att ändra i produkt"),
                    column![
                        Text::new("Lösendord"),
                        TextInput::new("", &self.password, Message::UpdatePassword)
                            .password()
                            .padding(DEF_PADDING)
                            .on_submit(Message::Login),
                        Button::new(Text::new("Logga In")).on_press(Message::Login),
                    ],
                )
                .max_width(650)
                .on_close(Message::CloseLogin)
                .into()
            },
        ))
        .map(Self::ExMessage::Manager)
    }
}
