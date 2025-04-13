use {
    super::{Message, Sideffect, TabId},
    crate::{
        icons::Icon,
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{padded_column, row, NumberInput, SquareButton, BIG_TEXT},
    },
    backend::items::{category::Category, Item},
    iced::{
        widget::{
            Button, Component, PickList, Responsive, Rule, Scrollable, Space, Text, TextInput,
        },
        Alignment, Element, Length, Size,
    },
    iced_aw::{Card, Modal, Wrap},
    strum::VariantArray,
};

pub struct Manager {
    menu: Vec<Item>,
}

#[derive(Debug, Clone, Default)]
pub enum Mode {
    #[default]
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

impl Component<Message> for Manager {
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::ToggleItem(i, a) => {
                if let Some(item) = self.menu.get(i) {
                    let item = item.clone();
                    return Some(
                        Sideffect::new(|| async move {
                            //TODO check if we can mutate item over thread boundary instead of
                            //realoading entire stock
                            item.change_availability(a).await?;
                            TabId::Manager.load().await
                        })
                        .into(),
                    );
                }
            }
            Event::EditItem(i) => {
                let item = &self.menu[i];
                state.mode = Mode::Update(item.name().clone());
                state.name = item.name().clone();
                state.price = *item.price();
                state.category = Some(*item.category());
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
                use std::mem::take;
                let name = take(&mut state.name);
                if !name.is_empty() {
                    let item = Item::new()
                        .with_name(name)
                        .with_price(take(&mut state.price))
                        .with_category(take(&mut state.category).unwrap_or_default());
                    return match std::mem::take(&mut state.mode) {
                        Mode::New => Some(
                            Sideffect::new(|| async move {
                                item.insert_new().await?;

                                //TODO see change_availability
                                TabId::Manager.load().await
                            })
                            .into(),
                        ),
                        Mode::Update(old_name) => {
                            let old = Item::new().with_name(old_name);
                            Some(
                                Sideffect::new(|| async move {
                                    old.update(item).await?;

                                    //TODO see change_availability
                                    TabId::Manager.load().await
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
            row![
                Responsive::new(|Size { width, .. }| {
                    Scrollable::new(
                        Wrap::with_elements(
                            self.menu
                                .iter()
                                .cloned()
                                .enumerate()
                                .map(|(i, item)| {
                                    crate::item::component::Item::from(item)
                                        .on_press(Event::EditItem(i))
                                        .on_toggle(move |b| Event::ToggleItem(i, b))
                                        .width(Length::Fixed(
                                            width / 3.0 - 2.0 * DEF_PADDING as f32,
                                        ))
                                        .into()
                                })
                                .collect(),
                        )
                        .spacing(DEF_PADDING as f32)
                        .line_spacing(DEF_PADDING as f32)
                        .padding(DEF_PADDING as f32),
                    )
                    .into()
                }),
                Rule::vertical(DEF_PADDING),
                padded_column![
                    row![
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
                    TextInput::new("", state.name.as_str())
                        .on_input(Event::UpdateName)
                        .padding(DEF_PADDING),
                    Text::new("Pris (kr)"),
                    NumberInput::new(1..=1000, Event::UpdatePrice, state.price),
                    Text::new("Typ"),
                    PickList::new(Category::VARIANTS, state.category, Event::UpdateCategory)
                        .width(Length::Fill),
                    Space::with_height(Length::FillPortion(5)),
                    if !state.locked {
                        Button::new(BIG_TEXT::new("Spara"))
                            .on_press(Event::Save)
                            .padding(DEF_PADDING)
                            .style(theme::Container::Border)
                            .width(Length::Fill)
                    } else {
                        Button::new(row![
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
                .width(Length::Fixed(RECEIPT_WIDTH)),
            ],
            state.login_modal.then(move || {
                Card::new(
                    Text::new("Login krävs för att ändra i produkt"),
                    padded_column![
                        Text::new("Lösendord"),
                        TextInput::new("", &password)
                            .on_input(Event::UpdatePassword)
                            .secure(true)
                            .padding(DEF_PADDING)
                            .on_submit(Event::Login),
                        Button::new(Text::new("Logga In"))
                            .style(theme::Container::Border)
                            .on_press(Event::Login),
                    ]
                    .height(Length::Shrink),
                )
                .max_width(650.0)
                .on_close(Event::CloseLogin)
            }),
        )
        .into()
    }
}

impl<'a> From<Manager> for Element<'a, Message> {
    fn from(manager: Manager) -> Self {
        iced::widget::component(manager)
    }
}
