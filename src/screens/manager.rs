use {
    super::{Message, Sideffect, Tab},
    crate::{
        icons::Icon,
        item::{category::Category, Item},
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{column, row, Grid, NumberInput, SquareButton, BIG_TEXT},
        Element, Renderer,
    },
    iced::{
        widget::{Button, PickList, Rule, Scrollable, Space, Text, TextInput},
        Alignment, Length,
    },
    iced_aw::{Card, Modal},
    iced_lazy::Component,
    rusqlite::params,
};

pub struct Manager {
    menu: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Mode {
    New,
    Update(String),
}

pub struct State {
    locked: bool,
    login_modal: bool,
    password: String,
    mode: Mode,
    name: String,
    price: i32,
    category: Option<Category>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            locked: true,
            login_modal: false,
            password: String::new(),
            mode: Mode::New,
            name: String::new(),
            price: 0,
            category: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    ToggleItem(usize, bool),
    EditItem(usize),
    UpdateName(String),
    UpdatePrice(i32),
    UpdateCategory(Category),
    Cancel,
    Save,
    OpenLogin,
    CloseLogin,
    UpdatePassword(String),
    Login,
}

impl Manager {
    pub fn new(menu: Vec<Item>) -> Self {
        Self { menu }
    }
}

impl Component<Message, Renderer> for Manager {
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::ToggleItem(i, a) => {
                if let Some(i) = self.menu.get(i) {
                    //Non breaking space gang
                    let name = i.name.replace(' ', "\u{00A0}");
                    return Some(
                        Sideffect::new(|| async move {
                            crate::DB.lock().await.execute(
                                "UPDATE menu SET available=?1 WHERE name=?2",
                                params![a, name],
                            )?;

                            Tab::Manager(vec![]).load().await
                        })
                        .into(),
                    );
                }
            }
            Event::EditItem(i) => {
                let item = &self.menu[i];
                state.mode = Mode::Update(item.name.clone());
                state.name = item.name.clone();
                state.price = item.price;
                state.category = Some(item.category);
            }
            Event::UpdateName(s) => state.name = s,
            Event::UpdatePrice(n) => state.price = n,
            Event::UpdateCategory(c) => state.category = Some(c),
            Event::Cancel => {
                state.mode = Mode::New;
                state.name.clear();
                state.price = 0;
            }
            Event::Save => {
                //Non breaking space gang
                let name = state.name.replace(' ', "\u{00A0}");
                let price = state.price;
                let category = state.category;
                if !name.is_empty() {
                    return match &state.mode {
                        Mode::New => Some(
                            Sideffect::new(|| async move {
                                crate::DB.lock().await.execute(
                                "INSERT INTO menu (name, price, available) VALUES (?1, ?2, true)",
                                params![name, price],
                            )?;

                                Tab::Manager(vec![]).load().await
                            })
                            .into(),
                        ),
                        Mode::Update(old_name) => {
                            let old_name = old_name.clone();
                            Some(
                                Sideffect::new(|| async move {
                                    crate::DB.lock().await.execute(
                                    "UPDATE menu SET name=?1, price=?2, category=?3 WHERE name=?4",
                                    params![name, price, category, old_name],
                                )?;

                                    Tab::Manager(vec![]).load().await
                                })
                                .into(),
                            )
                        }
                    };
                }
            }
            Event::OpenLogin => state.login_modal = true,
            Event::CloseLogin => state.login_modal = false,
            Event::UpdatePassword(password) => {
                state.password = password;
            }
            //No password in debug mode
            #[cfg(debug_assertions)]
            Event::Login => {
                state.locked = false;
                state.login_modal = false;
            }
            //Use env for password
            #[cfg(not(debug_assertions))]
            Event::Login => {
                state.locked = state.password != env!("SMALANDS_PASSWORD");
                state.login_modal = false;
            }
        }
        None
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event> {
        let password = state.password.clone();
        Modal::new(
            state.login_modal,
            row![
                #nopad
                Scrollable::new(
                    Grid::with_children(
                        self.menu.len() as u32 / 3,
                        3,
                        self.menu
                            .iter()
                            .cloned()
                            .enumerate()
                            .map(|(i ,item)| {
                                item
                                .on_press(Event::EditItem(i))
                                .on_toggle(move |b| Event::ToggleItem(i, b))
                                .into()
                            })
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
                        BIG_TEXT::new(match &state.mode {
                            Mode::New => String::from("Ny"),
                            Mode::Update(v) => {
                                format!("Ändrar {v}")
                            }
                        }),
                        Space::with_width(Length::Fill),
                        SquareButton::icon(Icon::Cross).on_press(Event::Cancel),
                    ]
                    .align_items(Alignment::Center),
                    Space::with_height(Length::FillPortion(1)),
                    Text::new("Namn"),
                    TextInput::new("", state.name.as_str(), Event::UpdateName)
                        .padding(DEF_PADDING),
                    Text::new("Pris (kr)"),
                    NumberInput::new(1..=1000, Event::UpdatePrice, Some(state.price)),
                    Text::new("Typ"),
                    PickList::new(&Category::ALL[..], state.category, Event::UpdateCategory),
                    Space::with_height(Length::FillPortion(5)),
                    if !state.locked {
                        Button::new(BIG_TEXT::new("Spara"))
                            .on_press(Event::Save)
                            .padding(DEF_PADDING)
                            .style(theme::Container::Border)
                            .width(Length::Fill)
                    } else {
                        Button::new(row![
                            #nopad
                            BIG_TEXT::new("Spara"),
                            Space::with_width(Length::Fill),
                            Icon::Lock,
                        ])
                        .on_press(Event::OpenLogin)
                        .padding(DEF_PADDING)
                        .style(theme::Container::Border)
                        .width(Length::Fill)
                    },
                ]
                .width(Length::Units(RECEIPT_WIDTH)),
            ],
            move || {
                Card::new(
                    Text::new("Login krävs för att ändra i produkt"),
                    column![
                        Text::new("Lösendord"),
                        TextInput::new("", &password, Event::UpdatePassword)
                            .password()
                            .padding(DEF_PADDING)
                            .on_submit(Event::Login),
                        Button::new(Text::new("Logga In"))
                            .style(theme::Container::Border)
                            .on_press(Event::Login),
                    ],
                )
                .max_width(650)
                .on_close(Event::CloseLogin)
                .into()
            },
        )
        .into()
    }
}

impl<'a> From<Manager> for Element<'a, Message> {
    fn from(manager: Manager) -> Self {
        iced_lazy::component(manager)
    }
}
