use {
    crate::{
        error::{Error, Result},
        grid::Grid,
        screens,
        screens::Screen,
        Marc, DEF_PADDING,
    },
    iced::{Column, Command, Element, Length, Row, Rule, Text},
    item::Item,
    rusqlite::{params, Connection},
    std::{collections::HashMap, future},
};

pub mod item;

pub struct Manager {
    menu: HashMap<String, Item>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    ToggleItem(String, bool),
    LoadMenu(Vec<Item>),
}

impl Screen for Manager {
    type ExMessage = screens::Message;
    type InMessage = Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                menu: HashMap::new(),
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
                                .prepare("SELECT name, price, available FROM menu")?
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
            Column::with_children(vec![])
                .width(Length::FillPortion(3))
                .spacing(DEF_PADDING)
                .padding(DEF_PADDING)
                .into(),
        ]))
        .map(Self::ExMessage::Manager)
    }
}
