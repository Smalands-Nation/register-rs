use {
    super::Screen,
    crate::{
        command_now,
        icons::Icon,
        item::Item,
        payment::Payment,
        print, query,
        receipt::Receipt,
        styles::{BIG_TEXT, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{
            calc::{self, Calc},
            Grid, SquareButton,
        },
    },
    chrono::Local,
    iced::{
        pure::{
            widget::{Button, Checkbox, Column, Container, Row, Rule, Scrollable, Space, Text},
            Pure, State,
        },
        Alignment, Command, Element, Length,
    },
    rusqlite::params,
    std::sync::Arc,
};

pub struct Menu {
    pure_state: State,
    calc: Calc,
    menu: Vec<Item>,
    receipt: Receipt,
    print: bool,
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
                pure_state: State::new(),
                calc: Calc::new(),
                menu: vec![],
                receipt: Receipt::new(Payment::Swish),
                print: false,
            },
            command_now!(Message::Refresh.into()),
        )
    }

    fn update(&mut self, message: Self::InMessage) -> Command<Self::ExMessage> {
        match message {
            Message::Refresh => {
                return query!(
                    "SELECT name, price, special FROM menu \
                    WHERE available=true ORDER BY special ASC, name DESC",
                    row => Item{
                        name: row.get::<usize, String>(0)?,
                        price: row.get(1)?,
                        num: (!row.get::<usize, bool>(2)?).then(|| 0),
                    },
                    Message::LoadMenu
                );
            }
            Message::Calc(m) => self.calc.update(m),
            Message::ClearReceipt => {
                self.receipt = Receipt::new(Payment::Swish);
            }
            Message::SellItem(mut i) => {
                i.num = i.num.map(|_| self.calc.0 as i32);
                self.receipt.add(i);
                self.calc.update(calc::Message::Clear);
            }
            Message::TogglePrint(b) => self.print = b,
            Message::Sell(p) => {
                let receipt = self.receipt.clone();
                let should_print = self.print;
                if !self.receipt.is_empty() {
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
                                    "INSERT INTO receipt_item (receipt, item, amount) \
                                    VALUES (?1, ?2, ?3)",
                                )?;

                                for item in receipt.items.values() {
                                    stmt.execute(params![
                                        time,
                                        item.name,
                                        item.num.unwrap_or(item.price)
                                    ])?;
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
        Into::<Element<Self::InMessage>>::into(Pure::new(
            &mut self.pure_state,
            Row::with_children(vec![
                Container::new(self.calc.view().map(Message::Calc))
                    .padding(DEF_PADDING)
                    .center_x()
                    .center_y()
                    .width(Length::Units(RECEIPT_WIDTH))
                    .height(Length::Fill)
                    .into(),
                Rule::vertical(DEF_PADDING).into(),
                Scrollable::new(
                    Grid::with_children(
                        self.menu.len() as u32 / 3,
                        3,
                        self.menu
                            .iter_mut()
                            .map(|i| i.as_widget().on_press(Message::SellItem).into())
                            .collect(),
                    )
                    .width(Length::Fill)
                    .spacing(DEF_PADDING)
                    .padding(DEF_PADDING),
                )
                //NOTE not available on pure .width(Length::Fill)
                //NOTE --||-- .spacing(DEF_PADDING)
                //NOTE --||-- .padding(DEF_PADDING)
                .into(),
                Rule::vertical(DEF_PADDING).into(),
                Column::with_children(vec![
                    Row::new()
                        .push(Text::new("Kvitto").size(BIG_TEXT))
                        .push(Space::with_width(Length::Fill))
                        .push(SquareButton::new(Icon::Cross).on_press(Message::ClearReceipt))
                        .align_items(Alignment::Center)
                        .into(),
                    self.receipt.as_widget().into(),
                    Checkbox::new(self.print, "Printa kvitto", Message::TogglePrint).into(),
                    Row::with_children(vec![
                        Button::new(Payment::Swish)
                            .on_press(Message::Sell(Payment::Swish))
                            .padding(DEF_PADDING)
                            .width(Length::Fill)
                            .into(),
                        Button::new(Payment::Paypal)
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
            ]),
        ))
        .map(Self::ExMessage::Menu)
    }
}
