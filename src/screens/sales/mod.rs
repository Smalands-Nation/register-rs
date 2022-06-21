use {
    super::Screen,
    crate::{
        command,
        error::Error,
        item::{Item, ItemKind},
        payment::Payment,
        receipt::Receipt,
        sql,
        styles::{BIG_TEXT, BORDERED, DEF_PADDING, RECEIPT_WIDTH, SMALL_TEXT},
    },
    chrono::{Date, Local, TimeZone},
    iced::{
        pure::{
            widget::{Button, Column, Container, Row, Rule, Space, Text},
            Element,
        },
        Alignment, Command, Length,
    },
    iced_aw::pure::date_picker::{self, DatePicker},
    indexmap::IndexMap,
    rusqlite::params,
};

mod save;

#[derive(Debug, Clone)]
pub enum Picker {
    From,
    To,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Load(Vec<(Item, Payment)>),
    Save,
    OpenDate(Picker),
    UpdateDate(date_picker::Date),
    CloseDate,
}

pub struct Sales {
    show_date: Option<Picker>,
    from: Date<Local>,
    to: Date<Local>,
    receipts: IndexMap<Payment, Receipt>,
}

impl Screen for Sales {
    type InMessage = Message;
    type ExMessage = super::Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                show_date: None,
                from: Local::today(),
                to: Local::today(),
                receipts: IndexMap::new(),
            },
            command!(Message::Refresh),
        )
    }

    fn update(&mut self, msg: Message) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh => {
                let from = self.from.and_hms(0, 0, 0);
                let to = self.to.and_hms(23, 59, 59);
                return sql!(
                    "SELECT item, amount, price, special, method FROM receipts_view \
                    WHERE time BETWEEN ?1 AND ?2",
                    params![from, to],
                    |row| {
                        Ok((
                            Item {
                                name: row.get("item")?,
                                price: row.get("price")?,
                                kind: if row.get("special")? {
                                    ItemKind::Special
                                } else {
                                    ItemKind::Regular {
                                        num: row.get("amount")?,
                                    }
                                },
                            },
                            //method
                            Payment::try_from(row.get::<usize, String>(4)?).unwrap_or_default(),
                        ))
                    },
                    Vec<(Item, Payment)>,
                    Message::Load
                );
            }
            Message::Load(map) => {
                self.receipts = map.into_iter().fold(
                    IndexMap::<_, Receipt, _>::new(),
                    |mut hm, (item, method)| {
                        match hm.get_mut(&method) {
                            Some(summary) => summary.add(item),
                            None => {
                                let mut summary = Receipt::new(method);
                                summary.add(item);
                                hm.insert(method, summary);
                            }
                        }
                        hm
                    },
                );
            }
            Message::Save => {
                let from = self.from;
                let to = self.to;
                let stats = self.receipts.clone();
                //Always return error to give info via modal
                return command!(if !stats.is_empty() {
                    match save::save(stats, (from, to)).await {
                        Ok(e) => Result::<(), Error>::Err(Error::Other(format!(
                            "Sparad till {}",
                            e.to_string_lossy()
                        ))),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(Error::Other("Ingen försäljning att spara".into()))
                });
            }
            Message::OpenDate(p) => {
                self.show_date = Some(p);
            }
            Message::UpdateDate(d) => {
                let date = Local.from_local_date(&d.into()).unwrap();
                match self.show_date {
                    Some(Picker::From) => {
                        self.from = date;
                    }
                    Some(Picker::To) => {
                        self.to = date;
                    }
                    None => (), //TODO logging here?
                };
                self.show_date = None;
                return command!(Message::Refresh);
            }
            Message::CloseDate => {
                self.show_date = None;
                return command!(Message::Refresh);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(DatePicker::new(
            self.show_date.is_some(),
            self.from.naive_local(),
            Row::with_children(vec![
                if !self.receipts.is_empty() {
                    self.receipts
                        .iter()
                        .fold(Row::new(), |row, (payment, rec)| {
                            row.push(
                                Container::new(
                                    Column::new()
                                        .push(Text::new(*payment).size(BIG_TEXT))
                                        .push(Space::new(Length::Fill, Length::Units(SMALL_TEXT)))
                                        .push(rec.as_widget())
                                        .width(Length::Units(RECEIPT_WIDTH))
                                        .padding(DEF_PADDING),
                                )
                                .style(BORDERED),
                            )
                        })
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .padding(DEF_PADDING)
                        .spacing(DEF_PADDING)
                        .into()
                } else {
                    Container::new(Text::new("Ingen försäljning än").size(BIG_TEXT))
                        .width(Length::Fill)
                        .center_x()
                        .padding(DEF_PADDING)
                        .into()
                },
                Rule::vertical(DEF_PADDING).into(),
                Column::with_children(vec![
                    Text::new("Visa Försäljning").size(BIG_TEXT).into(),
                    Space::with_height(Length::Fill).into(),
                    Text::new("Fr.o.m.").into(),
                    Button::new(Text::new(self.from.to_string()))
                        .on_press(Message::OpenDate(Picker::From))
                        .into(),
                    Text::new("T.o.m.").into(),
                    Button::new(Text::new(self.to.to_string()))
                        .on_press(Message::OpenDate(Picker::To))
                        .into(),
                    Space::with_height(Length::Fill).into(),
                    Button::new(Text::new("Exportera").size(BIG_TEXT))
                        .on_press(Message::Save)
                        .padding(DEF_PADDING)
                        .width(Length::Fill)
                        .into(),
                ])
                .width(Length::Units(RECEIPT_WIDTH))
                .padding(DEF_PADDING)
                .spacing(DEF_PADDING)
                .into(),
            ]),
            Message::CloseDate,
            Message::UpdateDate,
        ))
        .map(Self::ExMessage::Sales)
    }
}
