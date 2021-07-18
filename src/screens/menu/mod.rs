pub use {
    super::{db, Screen},
    crate::{
        error::{Error, Result},
        icons::Icon,
        payment::Payment,
        print,
        styles::{BIG_TEXT, DEF_PADDING, DEF_TEXT, RECIEPT_WIDTH},
        widgets::{
            calc::{self, Calc},
            Grid, Reciept, SquareButton,
        },
    },
    chrono::Local,
    iced::{
        button::{self, Button},
        scrollable::{self, Scrollable},
        window, Align, Application, Checkbox, Clipboard, Column, Command, Container, Element, Font,
        HorizontalAlignment, Length, Row, Rule, Settings, Space, Text,
    },
    indexmap::IndexMap,
    rusqlite::params,
    std::{future, sync::Arc},
};

pub mod item;
pub use item::Item;

pub struct Menu {
    calc: Calc,
    menu: Vec<Item>,
    reciept: Reciept<Message>,
    clear: button::State,
    print: bool,
    cash: button::State,
    swish: button::State,
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
    type ExMessage = super::Message;
    type InMessage = Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                calc: Calc::new(),
                menu: vec![],
                reciept: Reciept::new(),
                clear: button::State::new(),
                print: false,
                cash: button::State::new(),
                swish: button::State::new(),
            },
            future::ready(Message::Refresh.into()).into(),
        )
    }

    fn update(&mut self, message: Self::InMessage) -> Command<Self::ExMessage> {
        match message {
            Message::Refresh => {
                return db(|con| {
                    Ok(Message::LoadMenu(
                            con.lock()
                                .unwrap()
                                .prepare("SELECT name, price FROM menu WHERE available=true ORDER BY name DESC")?
                                .query_map(params![], |row| {
                                    Ok(Item::new(
                                        row.get::<usize, String>(0)?.as_str(),
                                        row.get(1)?,
                                        false,
                                    ))
                                })?
                                .map(|item| item.unwrap())
                                .collect(),
                        ).into())
                })
            }
            Message::Calc(m) => self.calc.update(m),
            Message::ClearReciept => {
                self.reciept = Reciept::new();
            }
            Message::SellItem(i) => {
                self.reciept.add(i.sell(self.calc.0 as i32));
                self.calc.update(calc::Message::Clear);
            }
            Message::TogglePrint(b) => self.print = b,
            Message::Sell(p) => {
                let reciept = (*self.reciept).clone();
                if self.reciept.len() > 0 {
                    return Command::perform(
                        print::print(reciept.clone(), Local::now()),
                        move |r| {
                            let reciept = reciept.clone();
                            match r {
                                Ok(_) => super::Message::DB(Arc::new(move |con| {
                                    con.lock()
                                        .unwrap()
                                        .execute(
                                            "INSERT INTO reciepts (time, items, sum, method) VALUES (?1, ?2, ?3, ?4)",
                                            params![Local::now(), reciept.json(), reciept.sum, String::from(p)]
                                        )?;
                                    Ok(Message::ClearReciept.into())
                                })),
                                Err(e) => super::Message::Error(e),
                            }
                        },
                    );
                }
            }
            Message::LoadMenu(mut menu) => {
                menu.append(&mut vec![
                    Item::new("Special", 1, true),
                    Item::new("Rabatt", -1, true),
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
                .width(Length::Units(RECIEPT_WIDTH))
                .height(Length::Fill)
                .into(),
            Rule::vertical(DEF_PADDING).into(),
            Grid::with_children(
                self.menu.len() as u32 / 3,
                3,
                self.menu.iter_mut().map(|i| i.view()).collect(),
            )
            .width(Length::Fill)
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
            Rule::vertical(DEF_PADDING).into(),
            Column::with_children(vec![
                Row::new()
                    .push(Text::new("Kvitto").size(BIG_TEXT))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        SquareButton::new(&mut self.clear, Text::from(Icon::Trash))
                            .on_press(Message::ClearReciept),
                    )
                    .align_items(Align::Center)
                    .into(),
                self.reciept.view(),
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
            .width(Length::Units(RECIEPT_WIDTH))
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
        ]))
        .map(Self::ExMessage::Menu)
    }
}
