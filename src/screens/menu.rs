use {
    super::Screen,
    crate::{
        command,
        icons::Icon,
        item::{kind::Sales, Item},
        payment::Payment,
        print,
        receipt::Receipt,
        sql,
        styles::{DEF_PADDING, RECEIPT_WIDTH},
        widgets::{calc::Calc, column, row, Grid, SquareButton, BIG_TEXT},
        Element,
    },
    chrono::Local,
    iced::{
        widget::{Button, Checkbox, Container, Rule, Scrollable, Space},
        Alignment, Command, Length,
    },
    rusqlite::params,
};

pub struct Menu {
    multiplier: u32,
    menu: Vec<Item<Sales>>,
    receipt: Receipt,
    print: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Multiplier(u32),
    SellItem(Item<Sales>),
    ClearReceipt,
    TogglePrint(bool),
    Sell(Payment),
    LoadMenu(Vec<Item<Sales>>),
}

impl Screen for Menu {
    type ExMessage = super::Message;
    type InMessage = Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                multiplier: 1,
                menu: vec![],
                receipt: Receipt::new(Payment::Swish),
                print: false,
            },
            command!(Message::Refresh),
        )
    }

    fn update(&mut self, message: Self::InMessage) -> Command<Self::ExMessage> {
        match message {
            Message::Refresh => {
                return sql!(
                    "SELECT name, price, special, category FROM menu \
                    WHERE available=true 
                    ORDER BY 
                        special ASC, 
                        CASE category 
                            WHEN 'alcohol' THEN 1
                            WHEN 'drink' THEN 2
                            WHEN 'food' THEN 3
                            WHEN 'other' THEN 4
                            ELSE 5
                        END,
                        name DESC",
                    params![],
                    Item::new_menu,
                    Vec<_>,
                    Message::LoadMenu
                );
            }
            Message::Multiplier(m) => {
                self.multiplier = m;
            }
            Message::ClearReceipt => {
                self.receipt = Receipt::new(Payment::Swish);
            }
            Message::SellItem(mut i) => {
                if let Some(0) = i.has_amount() {
                    i.set_amount(self.multiplier as i32);
                }
                self.receipt.add(i);
                self.multiplier = 1;
            }
            Message::TogglePrint(b) => self.print = b,
            Message::Sell(p) => {
                let receipt1 = self.receipt.clone();
                let receipt2 = self.receipt.clone();
                let should_print = self.print;
                if !self.receipt.is_empty() {
                    return Command::batch([
                        command!({
                            if should_print {
                                print::print(receipt1, Local::now())
                                    .await
                                    .map(|_| Message::Refresh)
                            } else {
                                Ok(Message::Refresh)
                            }
                        }),
                        command!({
                            let time = Local::now();

                            let con = crate::DB.lock().await;

                            con.execute(
                                "INSERT INTO receipts (time, method) VALUES (?1, ?2)",
                                params![time, String::from(p)],
                            )?;

                            let mut stmt = con.prepare(
                                "INSERT INTO receipt_item (receipt, item, amount, price) \
                                            VALUES (?1, ?2, ?3, ?4)",
                            )?;

                            for item in receipt2.items.iter() {
                                stmt.execute(params![
                                    time,
                                    item.name,
                                    item.has_amount().unwrap_or(0), //Special item has no ammount
                                    item.price,
                                ])?;
                            }

                            Ok(Message::ClearReceipt)
                        }),
                    ]);
                }
            }
            Message::LoadMenu(menu) => {
                self.menu = menu;
            }
        };
        Command::none()
    }

    fn view(&self) -> Element<Self::ExMessage> {
        Into::<Element<Self::InMessage>>::into(row![
            #nopad
            Container::new(Calc::new(self.multiplier ,Message::Multiplier))
                .padding(DEF_PADDING)
                .center_x()
                .center_y()
                .width(Length::Units(RECEIPT_WIDTH))
                .height(Length::Fill),
            Rule::vertical(DEF_PADDING),
            Scrollable::new(
                Grid::with_children(
                    self.menu.len() as u32 / 3,
                    3,
                    self.menu
                        .iter()
                        .map(|i| i.on_press(Message::SellItem).into())
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
                    BIG_TEXT::new("Kvitto"),
                    Space::with_width(Length::Fill),
                    SquareButton::icon(Icon::Cross).on_press(Message::ClearReceipt),
                ]
                .align_items(Alignment::Center),
                self.receipt.as_widget(),
                Checkbox::new(self.print, "Printa kvitto", Message::TogglePrint),
                row![
                    #nopad
                    Button::new(Payment::Swish)
                        .on_press(Message::Sell(Payment::Swish))
                        .padding(DEF_PADDING)
                        .width(Length::Fill),
                    Button::new(Payment::Paypal)
                        .on_press(Message::Sell(Payment::Paypal))
                        .padding(DEF_PADDING)
                        .width(Length::Fill),
                ]
                .spacing(DEF_PADDING)
            ]
            .width(Length::Units(RECEIPT_WIDTH)),
        ])
        .map(Self::ExMessage::Menu)
    }
}
