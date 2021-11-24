use {
    super::{db, Screen},
    crate::{
        payment::Payment,
        screens::transactions::Item,
        styles::{BIG_TEXT, BORDERED, DEF_PADDING, RECEIPT_WIDTH, SMALL_TEXT},
        widgets::Receipt,
    },
    iced::{button, Align, Column, Command, Container, Element, Length, Row, Space, Text},
    indexmap::IndexMap,
    rusqlite::params,
    std::future,
};

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Load(IndexMap<Payment, Receipt<Message>>),
    Save,
}

pub struct Sales {
    save: button::State,
    receipts: IndexMap<Payment, Receipt<Message>>,
}

impl Screen for Sales {
    type InMessage = Message;
    type ExMessage = super::Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                save: button::State::new(),
                receipts: IndexMap::new(),
            },
            future::ready(Message::Refresh.into()).into(),
        )
    }

    fn update(&mut self, msg: Message) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh => {
                return db(|con| {
                    Ok(Message::Load(
                            con.lock()
                                .unwrap()
                                .prepare("SELECT item, amount, price, special, method FROM receipts_view WHERE time > date('now','-1 day')")? //All receipts from last 24h, potentially bad for performance
                                .query_map(params![], |row| {
                                    Ok((
                                        //God hates me so all of these are type annotated
                                        //item
                                        row.get::<usize, String>(0)?,
                                        //amount
                                        row.get::<usize, i32>(1)?,
                                        //price
                                        row.get::<usize, i32>(2)?,
                                        //special
                                        row.get::<usize, bool>(3)?,
                                        //method
                                        Payment::try_from(row.get::<usize, String>(4)?).unwrap_or_default(),
                                    ))
                                })?
                                .map(|row| row.unwrap())
                                .map(|(item, num, price, special, method)| ( match (item, special) {
                                    (name, true) => Item::Special{name, price: num},
                                    (name, false) => Item::Regular{name, price, num},
                                }, method))
                                .fold(IndexMap::new(), |mut hm, (item, method)| {
                                    match hm.get_mut(&method) {
                                        Some(summary) => summary.add(item),
                                        None => {
                                            let mut summary = Receipt::new(method);
                                            summary.add(item);
                                            hm.insert(method, summary);
                                            }
                                        }
                                    hm
                                }),
                        ).into())
                });
            }
            Message::Load(map) => self.receipts = map,
            Message::Save => (),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(
            self.receipts
                .iter_mut()
                .fold(Row::new(), |row, (payment, rec)| {
                    row.push(
                        Container::new(
                            Column::new()
                                .push(Text::new(*payment).size(BIG_TEXT))
                                .push(Space::new(Length::Fill, Length::Units(SMALL_TEXT)))
                                .push(rec.view())
                                .width(Length::Units(RECEIPT_WIDTH))
                                .padding(DEF_PADDING),
                        )
                        .style(BORDERED),
                    )
                })
                .width(Length::Fill)
                .align_items(Align::Center)
                .padding(DEF_PADDING)
                .spacing(DEF_PADDING),
        )
        .map(Self::ExMessage::Sales)
    }
}
