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
        Align, Column, Command, Element, Length, Row, Rule, Space, Text,
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
    Refresh,
    ToggleItem(String, bool),
    LoadMenu(Vec<Item>),
    EditItem(Item),
    UpdateName(String),
    UpdatePrice(Option<u32>),
    Cancel,
    Save,
}

impl Screen for Manager {
    type ExMessage = super::Message;
    type InMessage = Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                menu: IndexMap::new(),
                mode: Mode::New,
                name: TextInput::new(),
                price: NumberInput::new(),
                cancel: button::State::new(),
                save: button::State::new(),
                scroll: scrollable::State::new(),
            },
            future::ready(Message::Refresh.into()).into(),
        )
    }

    fn update(&mut self, msg: Self::InMessage) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh => {
                return Command::batch([
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
                ]);
            }
            Message::ToggleItem(name, a) => match self.menu.get_mut(&name) {
                Some(i) => {
                    i.available = a;
                    let clone = i.clone();
                    return db(move |con| {
                        con.lock().unwrap().execute(
                            "UPDATE menu SET available=?1 WHERE name=?2",
                            //Non breaking space gang
                            params![clone.available, clone.name.replace(" ", "\u{00A0}")],
                        )?;
                        Ok(Message::Refresh.into())
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
                return match &self.mode {
                    Mode::New => db(move |con| {
                        con.lock().unwrap().execute(
                            "INSERT INTO menu (name, price, available) VALUES (?1, ?2, true)",
                            //Non breaking space gang
                            params![name.replace(" ", "\u{00A0}"), price],
                        )?;
                        Ok(Message::Refresh.into())
                    }),
                    Mode::Update(old_name) => {
                        let old_name = old_name.clone();
                        db(move |con| {
                            con.lock().unwrap().execute(
                                "UPDATE menu SET name=?1, price=?2 WHERE name=?3",
                                //Non breaking space gang
                                params![name.replace(" ", "\u{00A0}"), price, old_name],
                            )?;
                            Ok(Message::Refresh.into())
                        })
                    }
                };
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(Row::with_children(vec![
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
                    .push(Column::with_children(match &self.mode {
                        Mode::New => {
                            vec![Text::new("Ny").size(BIG_TEXT).into(), Text::new(" ").into()]
                        }
                        Mode::Update(v) => vec![
                            Text::new("Ändrar").size(BIG_TEXT).into(),
                            Text::new(v).into(),
                        ],
                    }))
                    .push(Space::with_width(Length::Fill))
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
                Button::new(&mut self.save, Text::new("Spara").size(BIG_TEXT))
                    .on_press(Message::Save)
                    .padding(DEF_PADDING)
                    .width(Length::Fill)
                    .into(),
            ])
            .width(Length::Units(RECEIPT_WIDTH))
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
        ]))
        .map(Self::ExMessage::Manager)
    }
}
