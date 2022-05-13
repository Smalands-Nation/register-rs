use {
    super::{db, Screen},
    crate::{
        command_now,
        icons::Icon,
        payment::Payment,
        print,
        receipt::Receipt,
        styles::{BORDERED, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{Clickable, SquareButton},
    },
    chrono::{DateTime, Local},
    iced::{
        pure::{
            widget::{Column, Container, Row, Rule, Space, Text},
            Pure, State,
        },
        Command, Element, Length,
    },
    indexmap::IndexMap,
    rusqlite::params,
};

pub mod item;
pub use item::Item;

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Init(IndexMap<DateTime<Local>, Receipt>),
    Append(IndexMap<DateTime<Local>, Receipt>),
    ScrollLeft,
    ScrollRight,
    Select(DateTime<Local>),
    Deselect,
    Print,
}

pub struct Transactions {
    pure_state: State,
    receipts: IndexMap<DateTime<Local>, Receipt>,
    selected: Option<(DateTime<Local>, Receipt)>,
    offset: usize,
}

impl Screen for Transactions {
    type InMessage = Message;
    type ExMessage = super::Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                pure_state: State::new(),
                receipts: IndexMap::new(),
                selected: None,
                offset: 0,
            },
            command_now!(Message::Refresh.into()),
        )
    }

    fn update(&mut self, msg: Message) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh => {
                return db(|con| {
                    Ok(Message::Init(
                        con.lock()
                            .unwrap()
                            .prepare(
                                "SELECT * FROM receipts_view \
                                    WHERE time > date('now','-1 day') ORDER BY time DESC",
                            )? //All receipts from last 24h, potentially bad for performance
                            .query_map(params![], |row| {
                                Ok((
                                    //God hates me so all of these are type annotated
                                    //time
                                    row.get::<usize, DateTime<Local>>(0)?,
                                    //item
                                    row.get::<usize, String>(1)?,
                                    //amount
                                    row.get::<usize, i32>(2)?,
                                    //price
                                    row.get::<usize, i32>(3)?,
                                    //special
                                    row.get::<usize, bool>(4)?,
                                    //method
                                    Payment::try_from(row.get::<usize, String>(5)?)
                                        .unwrap_or_default(),
                                ))
                            })?
                            .map(|res| {
                                res.map(|(time, item, num, price, special, method)| {
                                    (
                                        time,
                                        match (item, special) {
                                            (name, true) => Item::Special { name, price: num },
                                            (name, false) => Item::Regular { name, price, num },
                                        },
                                        method,
                                    )
                                })
                            })
                            .fold(Ok(IndexMap::<_, Receipt, _>::new()), |hm, res| {
                                hm.and_then(|mut hm| {
                                    res.map(|(time, item, method)| {
                                        match hm.get_mut(&time) {
                                            Some(receipt) => (*receipt).add(item),
                                            None => {
                                                let mut receipt = Receipt::new(method);
                                                receipt.add(item);
                                                hm.insert(time, receipt);
                                            }
                                        }
                                        hm
                                    })
                                })
                            })?,
                    )
                    .into())
                });
            }
            Message::Init(map) => self.receipts = map,
            Message::Append(map) => self.receipts.extend(map),
            Message::ScrollLeft if self.offset > 0 => self.offset -= 1,
            Message::ScrollRight
                if !self.receipts.is_empty() && self.offset < (self.receipts.len() - 1) / 3 =>
            {
                self.offset += 1
            }
            Message::Select(time) => {
                self.selected = self
                    .receipts
                    .get_key_value(&time)
                    .map(|(k, v)| (*k, v.clone()));
            }
            Message::Deselect => self.selected = None,
            Message::Print => {
                if let Some((time, receipt)) = &self.selected {
                    return Command::perform(
                        print::print((*receipt).clone(), *time),
                        |r| match r {
                            Ok(_) => Message::Deselect.into(),
                            Err(e) => super::Message::Error(e),
                        },
                    );
                }
            }
            _ => (),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Into::<Element<Self::InMessage>>::into(Pure::new(
            &mut self.pure_state,
            Row::new()
                .push(
                    Container::new(
                        self.receipts
                            .iter_mut()
                            .skip(self.offset * 3)
                            .take(3)
                            .fold(
                                Row::new().push(
                                    Clickable::new(
                                        Container::new(Text::from(Icon::Left))
                                            .width(Length::Fill)
                                            .height(Length::Fill)
                                            .center_x()
                                            .center_y(),
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .on_press(Message::ScrollLeft),
                                ),
                                |row, (t, rec)| {
                                    row.push(
                                        Container::new(
                                            rec.as_widget().on_press(Message::Select(*t)),
                                        )
                                        .padding(DEF_PADDING)
                                        .style(BORDERED),
                                    )
                                },
                            )
                            .push(
                                Clickable::new(
                                    Container::new(Text::from(Icon::Right))
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .center_x()
                                        .center_y(),
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .on_press(Message::ScrollRight),
                            )
                            .padding(DEF_PADDING)
                            .spacing(DEF_PADDING),
                    )
                    .center_x()
                    .width(Length::Fill),
                )
                .push(Rule::vertical(DEF_PADDING))
                .push(
                    match &mut self.selected {
                        Some((_, rec)) => Column::new().push(rec.as_widget()),
                        None => Column::new()
                            .push(Space::new(Length::Units(RECEIPT_WIDTH), Length::Fill)),
                    }
                    .push(
                        Row::new()
                            .push(
                                SquareButton::new(Text::from(Icon::Cross))
                                    .on_press(Message::Deselect),
                            )
                            .push(Space::with_width(Length::Fill))
                            .push(
                                SquareButton::new(Text::from(Icon::Print)).on_press(Message::Print),
                            )
                            .padding(DEF_PADDING)
                            .spacing(DEF_PADDING),
                    )
                    .padding(DEF_PADDING)
                    .width(Length::Units(RECEIPT_WIDTH)),
                ),
        ))
        .map(Self::ExMessage::Transactions)
    }
}
