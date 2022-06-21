use {
    super::Screen,
    crate::{
        command,
        icons::Icon,
        item::{Item, ItemKind},
        payment::Payment,
        print,
        receipt::Receipt,
        sql,
        styles::{BORDERED, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{Clickable, SquareButton},
    },
    chrono::{DateTime, Local},
    iced::{
        pure::{
            widget::{Column, Container, Row, Rule, Space},
            Element,
        },
        Command, Length,
    },
    indexmap::IndexMap,
    rusqlite::params,
};

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Init(Vec<(DateTime<Local>, Item, Payment)>),
    Append(IndexMap<DateTime<Local>, Receipt>),
    ScrollLeft,
    ScrollRight,
    Select(DateTime<Local>),
    Deselect,
    Print,
}

pub struct Transactions {
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
                receipts: IndexMap::new(),
                selected: None,
                offset: 0,
            },
            command!(Message::Refresh),
        )
    }

    fn update(&mut self, msg: Message) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh => {
                return sql!(
                    "SELECT * FROM receipts_view \
                    WHERE time > date('now','-1 day') ORDER BY time DESC",
                    params![],
                    |row| {
                        Ok((
                            row.get::<_, DateTime<Local>>("time")?,
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
                            Payment::try_from(row.get::<usize, String>(5)?).unwrap_or_default(),
                        ))
                    },
                    Vec<_>,
                    Message::Init
                );
            }
            Message::Init(map) => {
                self.receipts = map.into_iter().fold(
                    IndexMap::<_, Receipt, _>::new(),
                    |mut hm, (time, item, method)| {
                        match hm.get_mut(&time) {
                            Some(receipt) => (*receipt).add(item),
                            None => {
                                let mut receipt = Receipt::new(method);
                                receipt.add(item);
                                hm.insert(time, receipt);
                            }
                        }
                        hm
                    },
                );
            }
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
                    let receipt = receipt.clone();
                    let time = *time;
                    return command!(print::print(receipt, time).await.map(|_| Message::Deselect));
                }
            }
            _ => (),
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::ExMessage> {
        Into::<Element<Self::InMessage>>::into(
            Row::new()
                .push(
                    Container::new(
                        self.receipts
                            .iter()
                            .skip(self.offset * 3)
                            .take(3)
                            .fold(
                                Row::new().push(
                                    Clickable::new(
                                        Container::new(Icon::Left)
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
                                    Container::new(Icon::Right)
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
                    match &self.selected {
                        Some((_, rec)) => Column::new().push(rec.as_widget()),
                        None => Column::new()
                            .push(Space::new(Length::Units(RECEIPT_WIDTH), Length::Fill)),
                    }
                    .push(
                        Row::new()
                            .push(SquareButton::new(Icon::Cross).on_press(Message::Deselect))
                            .push(Space::with_width(Length::Fill))
                            .push(SquareButton::new(Icon::Print).on_press(Message::Print))
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
