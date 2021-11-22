use {
    super::{db, Screen},
    crate::{
        icons::Icon,
        payment::Payment,
        print,
        styles::{BORDERED, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{Clickable, Receipt, SquareButton},
    },
    chrono::{DateTime, Local},
    iced::{button, Column, Command, Container, Element, Length, Row, Rule, Space, Text},
    indexmap::IndexMap,
    rusqlite::params,
    std::future,
};

pub mod item;
pub use item::Item;

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Init(IndexMap<DateTime<Local>, Receipt<Message>>),
    Append(IndexMap<DateTime<Local>, Receipt<Message>>),
    ScrollLeft,
    ScrollRight,
    Select(DateTime<Local>),
    Deselect,
    Print,
}

pub struct Transactions {
    left: button::State,
    right: button::State,
    deselect: button::State,
    print: button::State,
    receipts: IndexMap<DateTime<Local>, Receipt<Message>>,
    selected: Option<(DateTime<Local>, Receipt<Message>)>,
    offset: usize,
}

impl Screen for Transactions {
    type InMessage = Message;
    type ExMessage = super::Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                left: button::State::new(),
                right: button::State::new(),
                deselect: button::State::new(),
                print: button::State::new(),
                receipts: IndexMap::new(),
                selected: None,
                offset: 0,
            },
            future::ready(Message::Refresh.into()).into(),
        )
    }

    fn update(&mut self, msg: Message) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh => {
                return db(|con| {
                    Ok(Message::Init(
                            con.lock()
                                .unwrap()
                                .prepare("SELECT * FROM receipts_view WHERE time > date('now','-1 day') ORDER BY time DESC")? //All receipts from last 24h, potentially bad for performance
                                .query_map(params![], |row| {
                                    Ok((
                                        //God hates me so all of these are type annotated
                                        //time
                                        row.get::<usize,DateTime<Local>>(0)?,
                                        //item
                                        row.get::<usize, String>(1)?,
                                        //amount
                                        row.get::<usize, i32>(2)?,
                                        //price
                                        row.get::<usize, i32>(3)?,
                                        //special
                                        row.get::<usize, bool>(4)?,
                                        //method
                                        match row.get::<usize, String>(5)?.as_str() {
                                            "Cash" => Payment::Cash,
                                            _ => Payment::Swish,
                                        },
                                    ))
                                })?
                                .map(|row| row.unwrap())
                                .map(|(time, item, num, price, special, method)| (time, match (item, special) {
                                    (name, true) => Item::Special{name, price: num},
                                    (name, false) => Item::Regular{name, price, num},
                                }, method))
                                .fold(IndexMap::new(), |mut hm, (time, item, method)| {
                                    match hm.get_mut(&time) {
                                        Some(receipt) => (*receipt).add(item),
                                        None => {
                                            let mut receipt = Receipt::new(method);
                                            receipt.add(item);
                                            hm.insert(time.clone(), receipt.on_press(Message::Select(time)));
                                            }
                                        }
                                    hm
                                }),
                        ).into())
                });
            }
            Message::Init(map) => self.receipts = map,
            Message::Append(map) => self.receipts.extend(map),
            Message::ScrollLeft if self.offset > 0 => self.offset -= 1,
            Message::ScrollRight
                if self.receipts.len() != 0 && self.offset < (self.receipts.len() - 1) / 3 =>
            {
                self.offset += 1
            }
            Message::Select(time) => {
                self.selected = self
                    .receipts
                    .get_key_value(&time)
                    .map(|(k, v)| (k.clone(), v.clone()));
            }
            Message::Deselect => self.selected = None,
            Message::Print => {
                if let Some((time, receipt)) = &self.selected {
                    return Command::perform(
                        print::print((**receipt).clone(), time.clone()),
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
        Element::<Self::InMessage>::from(
            Row::new()
                .push(
                    Container::new(
                        self.receipts
                            .values_mut()
                            .skip(self.offset * 3)
                            .take(3)
                            .fold(
                                Row::new().push(
                                    Clickable::new(
                                        &mut self.left,
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
                                |row, rec| {
                                    row.push(
                                        Container::new(rec.view())
                                            .padding(DEF_PADDING)
                                            .style(BORDERED),
                                    )
                                },
                            )
                            .push(
                                Clickable::new(
                                    &mut self.right,
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
                        Some((_, rec)) => Column::new().push(rec.view()),
                        None => Column::new()
                            .push(Space::new(Length::Units(RECEIPT_WIDTH), Length::Fill)),
                    }
                    .push(
                        Row::new()
                            .push(
                                SquareButton::new(&mut self.deselect, Text::from(Icon::Cross))
                                    .on_press(Message::Deselect),
                            )
                            .push(Space::with_width(Length::Fill))
                            .push(
                                SquareButton::new(&mut self.print, Text::from(Icon::Print))
                                    .on_press(Message::Print),
                            )
                            .padding(DEF_PADDING)
                            .spacing(DEF_PADDING),
                    )
                    .padding(DEF_PADDING)
                    .width(Length::Units(RECEIPT_WIDTH)),
                ),
        )
        .map(Self::ExMessage::Transactions)
    }
}
