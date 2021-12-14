use {
    super::{db, Screen},
    crate::{
        icons::Icon,
        payment::Payment,
        print,
        styles::{BIG_TEXT, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{
            calc::{self, Calc},
            Grid, Receipt, SquareButton,
        },
    },
    chrono::Local,
    iced::{
        button::{self, Button},
        scrollable::{self, Scrollable},
        Align, Canvas, Checkbox, Column, Command, Container, Element, Length, Row, Rule, Space,
        Text,
    },
    rusqlite::params,
    std::{future, sync::Arc},
};

pub mod item;
pub use item::Item;

pub struct Menu {
    calc: Calc,
    menu: Vec<Item>,
    receipt: Receipt<Message>,
    clear: button::State,
    print: bool,
    swish: button::State,
    paypal: button::State,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Calc(calc::Message),
    SellItem(Item),
    ClearReceipt,
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
                receipt: Receipt::new(Payment::Swish),
                clear: button::State::new(),
                print: false,
                swish: button::State::new(),
                paypal: button::State::new(),
                scroll: scrollable::State::new(),
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
                                .prepare("SELECT name, price, special FROM menu WHERE available=true ORDER BY special ASC, name DESC")?
                                .query_map(params![], |row| {
                                    Ok(Item::new(
                                        row.get::<usize, String>(0)?.as_str(),
                                        row.get(1)?,
                                        row.get(2)?,
                                    ))
                                })?
                                .map(|item| item.unwrap())
                                .collect(),
                        ).into())
                })
            }
            Message::Calc(m) => self.calc.update(m),
            Message::ClearReceipt => {
                self.receipt = Receipt::new(Payment::Swish);
            }
            Message::SellItem(i) => {
                self.receipt.add(i.sell(self.calc.0 as i32));
                self.calc.update(calc::Message::Clear);
            }
            Message::TogglePrint(b) => self.print = b,
            Message::Sell(p) => {
                let receipt = (*self.receipt).clone();
                let should_print = self.print;
                if self.receipt.len() > 0 {
                    return Command::perform(
                        async move {
                            if should_print {
                                print::print(receipt, Local::now()).await
                            } else {
                                Ok(receipt)
                            }
                        },
                        move |r| match r {
                            Ok(receipt) => super::Message::DB(Arc::new(move |con| {
                                let time = Local::now();

                                let con = con.lock().unwrap();

                                con.execute(
                                    "INSERT INTO receipts (time, method) VALUES (?1, ?2)",
                                    params![time, String::from(p)],
                                )?;

                                let mut stmt = con.prepare(
                                        "INSERT INTO receipt_item (receipt, item, amount) VALUES (?1, ?2, ?3)",
                                    )?;

                                for item in receipt.items.values() {
                                    stmt.execute(params![time, item.name(), item.num()])?;
                                }

                                Ok(Message::ClearReceipt.into())
                            })),
                            Err(e) => super::Message::Error(e),
                        },
                    );
                }
            }
            Message::LoadMenu(menu) => {
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
                .width(Length::Units(RECEIPT_WIDTH))
                .height(Length::Fill)
                .into(),
            Rule::vertical(DEF_PADDING).into(),
            Scrollable::new(&mut self.scroll)
                .push(
                    Grid::with_children(
                        self.menu.len() as u32 / 3,
                        3,
                        self.menu.iter_mut().map(|i| i.view()).collect(),
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
                    .push(Text::new("Kvitto").size(BIG_TEXT))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        SquareButton::new(&mut self.clear, Text::from(Icon::Trash))
                            .on_press(Message::ClearReceipt),
                    )
                    .align_items(Align::Center)
                    .into(),
                self.receipt.view(),
                Checkbox::new(self.print, "Printa kvitto", Message::TogglePrint).into(),
                Row::with_children(vec![
                    Button::new(&mut self.swish, Canvas::new(Payment::Swish))
                        .on_press(Message::Sell(Payment::Swish))
                        .padding(DEF_PADDING)
                        .width(Length::Fill)
                        .into(),
                    Button::new(&mut self.paypal, Canvas::new(Payment::Paypal))
                        .on_press(Message::Sell(Payment::Paypal))
                        .padding(DEF_PADDING)
                        .width(Length::Fill)
                        .into(),
                ])
                .spacing(DEF_PADDING)
                .into(),
            ])
            .width(Length::Units(RECEIPT_WIDTH))
            .spacing(DEF_PADDING)
            .padding(DEF_PADDING)
            .into(),
        ]))
        .map(Self::ExMessage::Menu)
    }
}
