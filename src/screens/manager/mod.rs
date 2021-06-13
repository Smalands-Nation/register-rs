use {
    crate::{
        error::{Error, Result},
        icons::Icon,
        screens,
        screens::Screen,
        widgets::{grid::Grid, numberinput::NumberInput, textinput::TextInput},
        Marc, BIG_TEXT, DEF_PADDING,
    },
    iced::{button, button::Button, Column, Command, Element, Length, Row, Rule, Space, Text},
    item::Item,
    rusqlite::{params, Connection},
    std::{collections::HashMap, future},
};

pub mod item;

#[derive(Debug, Clone)]
pub enum Mode {
    New,
    Update(String),
}

pub struct Manager {
    menu: HashMap<String, Item>,
    mode: Mode,
    name: TextInput,
    price: NumberInput<u32>,
    cancel: button::State,
    save: button::State,
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
    type ExMessage = screens::Message;
    type InMessage = Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                menu: HashMap::new(),
                mode: Mode::New,
                name: TextInput::new(),
                price: NumberInput::new(),
                cancel: button::State::new(),
                save: button::State::new(),
            },
            Command::perform(future::ready(()), |_| {
                Self::ExMessage::Manager(Message::Refresh)
            }),
        )
    }

    fn update(&mut self, msg: Self::InMessage) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh => {
                return Command::perform(
                    future::ready::<fn(Marc<Connection>) -> Result<Self::ExMessage>>(|con| {
                        Ok(Self::ExMessage::Manager(Message::LoadMenu(
                            con.lock()
                                .unwrap()
                                .prepare(
                                    "SELECT name, price, available FROM menu ORDER BY name DESC",
                                )?
                                .query_map(params![], |row| {
                                    Ok(Item::new(
                                        row.get::<usize, String>(0)?.as_str(),
                                        row.get(1)?,
                                        row.get(2)?,
                                    ))
                                })?
                                .map(|item| item.unwrap())
                                .collect(),
                        )))
                    }),
                    Self::ExMessage::ReadDB,
                )
            }
            Message::ToggleItem(name, a) => match self.menu.get_mut(&name) {
                Some(i) => {
                    i.available = a;
                    return Command::perform(
                        future::ready(format!(
                            "UPDATE menu SET available={} WHERE name=\"{}\"",
                            i.available, i.name,
                        )),
                        Self::ExMessage::WriteDB,
                    );
                }
                None => (),
            },
            Message::LoadMenu(m) => {
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
                return Command::batch(vec![
                    match &self.mode {
                        Mode::New => Command::perform(
                            future::ready(format!(
                            "INSERT INTO menu (name, price, available) VALUES (\"{}\", {}, true)",
                            self.name.value(),
                            self.price.value().unwrap_or(0),
                        )),
                            Self::ExMessage::WriteDB,
                        ),
                        Mode::Update(name) => Command::perform(
                            future::ready(format!(
                                "UPDATE menu SET name=\"{}\", price={} WHERE name=\"{}\"",
                                self.name.value(),
                                self.price.value().unwrap_or(0),
                                name,
                            )),
                            Self::ExMessage::WriteDB,
                        ),
                    },
                    Command::perform(future::ready(()), |_| {
                        Self::ExMessage::Manager(Message::Refresh)
                    }),
                ]);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(Row::with_children(vec![
            Grid::with_children(
                self.menu.len() as u32 / 3,
                3,
                self.menu.iter_mut().map(|(_, i)| i.view()).collect(),
            )
            .width(Length::FillPortion(8))
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
            Rule::vertical(DEF_PADDING).into(),
            Column::with_children(vec![
                Row::new()
                    .push(
                        Text::new(format!(
                            "{} Vara",
                            match self.mode {
                                Mode::New => "Ny",
                                Mode::Update(_) => "Ändra",
                            }
                        ))
                        .size(BIG_TEXT),
                    )
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Button::new(&mut self.cancel, Text::from(Icon::Cross))
                            .on_press(Message::Cancel),
                    )
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
            .width(Length::FillPortion(3))
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
        ]))
        .map(Self::ExMessage::Manager)
    }
}
