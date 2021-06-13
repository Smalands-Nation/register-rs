pub use {
    crate::{
        error::Error,
        error::Result,
        icons::Icon,
        screens,
        screens::Screen,
        widgets::{calc, calc::Calc, grid::Grid},
        Marc, BIG_TEXT, DEF_PADDING, DEF_TEXT,
    },
    iced::{
        button, scrollable, window, Application, Button, Checkbox, Clipboard, Column, Command,
        Container, Element, Font, HorizontalAlignment, Length, Row, Rule, Scrollable, Settings,
        Space, Text,
    },
    item::Item,
    rusqlite::{params, Connection},
    std::{collections::HashMap, future, sync::Arc},
};

pub mod item;

pub struct Menu {
    calc: Calc,
    menu: Vec<Item>,
    reciept: HashMap<Item, Item>,
    scroll: scrollable::State,
    clear: button::State,
    total: i32,
    print: bool,
    cash: button::State,
    swish: button::State,
}

#[derive(Debug, Clone)]
pub enum Payment {
    Cash,
    Swish,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Calc(calc::Message),
    SellItem(Item),
    ClearReciept,
    TogglePrint(bool),
    Sell(Payment),
    LoadMenu(Vec<Item>),
}

impl Screen for Menu {
    type ExMessage = screens::Message;
    type InMessage = Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                calc: Calc::new(),
                menu: vec![],
                reciept: HashMap::new(),
                scroll: scrollable::State::new(),
                clear: button::State::new(),
                total: 0,
                print: false,
                cash: button::State::new(),
                swish: button::State::new(),
            },
            Command::perform(future::ready(()), |_| {
                Self::ExMessage::Menu(Message::Refresh)
            }),
        )
    }

    fn update(&mut self, message: Self::InMessage) -> Command<Self::ExMessage> {
        match message {
            Message::Refresh => {
                return Command::perform(
                    future::ready::<fn(Marc<Connection>) -> Result<Self::ExMessage>>(|con| {
                        Ok(Self::ExMessage::Menu(Message::LoadMenu(
                            con.lock()
                                .unwrap()
                                .prepare("SELECT name, price FROM menu WHERE available=true ORDER BY name DESC")?
                                .query_map(params![], |row| {
                                    Ok(Item::new(
                                        row.get::<usize, String>(0)?.as_str(),
                                        row.get(1)?,
                                    ))
                                })?
                                .map(|item| item.unwrap())
                                .collect(),
                        )))
                    }),
                    Self::ExMessage::ReadDB,
                )
            }
            Message::Calc(m) => self.calc.update(m),
            Message::ClearReciept => {
                self.reciept = HashMap::new();
                self.total = 0;
            }
            Message::SellItem(i) => {
                let i = i.sell(self.calc.0 as i32);
                match self.reciept.get_mut(&i) {
                    Some(it) => {
                        *it = match (i, it.clone()) {
                            (Item::Sold(n1, p1, x1), Item::Sold(n2, p2, x2))
                                if n1 == n2 && p1 == p2 =>
                            {
                                Item::Sold(n1, p1, x1 + x2)
                            }
                            (Item::SoldSpecial(n1, p1), Item::SoldSpecial(n2, p2)) if n1 == n2 => {
                                Item::SoldSpecial(n1, p1 + p2)
                            }
                            (_, it @ _) => it,
                        };
                    }
                    None => {
                        self.reciept.insert(i.clone(), i);
                    }
                }
                self.total = self.reciept.values_mut().fold(0i32, |acc, n| {
                    acc + match n {
                        Item::Sold(_, price, num) => *price * *num,
                        Item::SoldSpecial(_, price) => *price,
                        _ => 0,
                    }
                });
                self.calc.update(calc::Message::Clear);
            }
            Message::TogglePrint(b) => self.print = b,
            Message::Sell(_p) => {
                let r: Vec<Item> = self.reciept.values().map(|v| v.clone()).collect();
                self.update(Message::ClearReciept);
                println!("{:?}", r);
            }
            Message::LoadMenu(mut menu) => {
                menu.append(&mut vec![
                    Item::new_special("Special", 1),
                    Item::new_special("Rabatt", -1),
                ]);
                self.menu = menu;
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(Row::with_children(vec![
            Container::new(self.calc.view().map(Message::Calc))
                .padding(DEF_PADDING)
                .center_x()
                .center_y()
                .width(Length::FillPortion(3))
                .height(Length::Fill)
                .into(),
            Rule::vertical(DEF_PADDING).into(),
            Grid::with_children(
                self.menu.len() as u32 / 3,
                3,
                self.menu.iter_mut().map(|i| i.view()).collect(),
            )
            .width(Length::FillPortion(8))
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
            Rule::vertical(DEF_PADDING).into(),
            Column::with_children(vec![
                Row::new()
                    .push(Text::new("Kvitto").size(BIG_TEXT))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Button::new(&mut self.clear, Text::from(Icon::Trash))
                            .on_press(Message::ClearReciept),
                    )
                    .into(),
                self.reciept
                    .values_mut()
                    .fold(
                        Scrollable::new(&mut self.scroll).spacing(DEF_PADDING),
                        |c, i| c.push(i.view()),
                    )
                    .height(Length::Fill)
                    .into(),
                Text::new(format!("Total: {}kr", self.total)).into(),
                Checkbox::new(self.print, "Printa kvitto", |b| Message::TogglePrint(b)).into(),
                Button::new(&mut self.cash, Text::new("Kontant").size(BIG_TEXT))
                    .on_press(Message::Sell(Payment::Cash))
                    .padding(DEF_PADDING)
                    .width(Length::Fill)
                    .into(),
                Button::new(&mut self.swish, Text::new("Swish").size(BIG_TEXT))
                    .on_press(Message::Sell(Payment::Swish))
                    .padding(DEF_PADDING)
                    .width(Length::Fill)
                    .into(),
            ])
            .width(Length::FillPortion(3))
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
        ]))
        .map(Self::ExMessage::Menu)
    }
}
