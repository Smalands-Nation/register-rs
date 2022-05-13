use {
    super::Screen,
    crate::{
        command_now,
        payment::Payment,
        query,
        screens::transactions::Item,
        styles::{BIG_TEXT, BORDERED, DEF_PADDING, RECEIPT_WIDTH, SMALL_TEXT},
        widgets::{DatePicker, Receipt},
    },
    chrono::{Local, NaiveDate, TimeZone},
    iced::{
        button::{self, Button},
        pure::{
            widget::{
                Column as PColumn, Container as PContainer, Row as PRow, Space as PSpace,
                Text as PText,
            },
            Pure, State,
        },
        Alignment, Column, Command, Container, Element, Length, Row, Rule, Space, Text,
    },
    iced_aw::date_picker::Date,
    indexmap::IndexMap,
    rusqlite::params,
};

#[derive(Debug, Clone)]
pub enum Picker {
    From,
    To,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Load(IndexMap<Payment, Receipt<Message>>),
    Save,
    OpenDate(Picker),
    UpdateDate(Picker, Date),
    CloseDate(Picker),
}

pub struct Sales {
    pure_state: State,
    from: DatePicker,
    to: DatePicker,
    save: button::State,
    receipts: IndexMap<Payment, Receipt<Message>>,
}

impl Screen for Sales {
    type InMessage = Message;
    type ExMessage = super::Message;

    fn new() -> (Self, Command<Self::ExMessage>) {
        (
            Self {
                pure_state: State::new(),
                from: DatePicker::new(),
                to: DatePicker::new(),
                save: button::State::new(),
                receipts: IndexMap::new(),
            },
            command_now!(Message::Refresh.into()),
        )
    }

    fn update(&mut self, msg: Message) -> Command<Self::ExMessage> {
        match msg {
            Message::Refresh => {
                let from = Local
                    .from_local_datetime(&NaiveDate::from(self.from.value()).and_hms(0, 0, 0))
                    .unwrap();
                let to = Local
                    .from_local_datetime(&NaiveDate::from(self.to.value()).and_hms(23, 59, 59))
                    .unwrap();
                return query!("SELECT item, amount, price, special, method FROM receipts_view \
                    WHERE time BETWEEN ?1 AND ?2",
                params![from, to],
                row => (
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
                ),
                Message::Load;
                iter.map(|res| res.map(
                                    |(item, num, price, special, method)| {
                                    (match (item, special) {
                                        (name, true) => Item::Special{name, price: num},
                                        (name, false) => Item::Regular{name, price, num},
                                    }, method)
                                },
                            ))
                            .fold(Ok(IndexMap::<_,Receipt<Message>,_>::new()), |hm, res| {
                                hm.and_then(|mut hm| {
                                    res.map(|( item, method)| {
                                        match hm.get_mut(&method) {
                                            Some(summary) => summary.add(item),
                                            None => {
                                                let mut summary = Receipt::new(method);
                                                summary.add(item);
                                                hm.insert(method, summary);
                                                }
                                            }
                                        hm
                                    })
                                })
                            })?
                );
            }
            Message::Load(map) => self.receipts = map,
            Message::Save => (),
            Message::OpenDate(p) => {
                let p = match p {
                    Picker::From => &mut self.from,
                    Picker::To => &mut self.to,
                };
                p.state.reset();
                p.state.show(true);
            }
            Message::UpdateDate(p, d) => {
                let p = match p {
                    Picker::From => &mut self.from,
                    Picker::To => &mut self.to,
                };
                p.update(d);
                p.state.show(false);
                return command_now!(Message::Refresh.into());
            }
            Message::CloseDate(p) => {
                match p {
                    Picker::From => &mut self.from,
                    Picker::To => &mut self.to,
                }
                .state
                .show(false);
                return command_now!(Message::Refresh.into());
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::ExMessage> {
        Element::<Self::InMessage>::from(Row::with_children(vec![
            if !self.receipts.is_empty() {
                Pure::new(
                    &mut self.pure_state,
                    self.receipts
                        .iter_mut()
                        .fold(PRow::new(), |row, (payment, rec)| {
                            row.push(
                                PContainer::new(
                                    PColumn::new()
                                        .push(PText::new(*payment).size(BIG_TEXT))
                                        .push(PSpace::new(Length::Fill, Length::Units(SMALL_TEXT)))
                                        .push(rec.view())
                                        .width(Length::Units(RECEIPT_WIDTH))
                                        .padding(DEF_PADDING),
                                )
                                .style(BORDERED),
                            )
                        })
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .padding(DEF_PADDING)
                        .spacing(DEF_PADDING),
                )
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
                self.from
                    .build(
                        Message::OpenDate(Picker::From),
                        Message::CloseDate(Picker::From),
                        |d| Message::UpdateDate(Picker::From, d),
                    )
                    .into(),
                Text::new("T.o.m.").into(),
                self.to
                    .build(
                        Message::OpenDate(Picker::To),
                        Message::CloseDate(Picker::To),
                        |d| Message::UpdateDate(Picker::To, d),
                    )
                    .into(),
                Space::with_height(Length::Fill).into(),
                Button::new(&mut self.save, Text::new("Exportera").size(BIG_TEXT))
                    .on_press(Message::Save)
                    .padding(DEF_PADDING)
                    .width(Length::Fill)
                    .into(),
            ])
            .width(Length::Units(RECEIPT_WIDTH))
            .padding(DEF_PADDING)
            .spacing(DEF_PADDING)
            .into(),
        ]))
        .map(Self::ExMessage::Sales)
    }
}
