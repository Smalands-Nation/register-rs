use {
    super::{Message, Sideffect, Tab},
    crate::{
        error::Error,
        item::Item,
        payment::Payment,
        receipt::Receipt,
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{column, row, BIG_TEXT, SMALL_TEXT},
        Element, Renderer,
    },
    chrono::{Date, Local, TimeZone},
    iced::{
        widget::{Button, Container, Row, Rule, Space, Text},
        Alignment, Length,
    },
    iced_aw::date_picker::{self, DatePicker},
    iced_lazy::Component,
    indexmap::IndexMap,
};

mod save;

#[derive(Debug, Clone)]
pub enum Picker {
    From,
    To,
}

pub struct Sales {
    from: Date<Local>,
    to: Date<Local>,
    receipts: IndexMap<Payment, Receipt<Event>>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Save,
    OpenDate(Picker),
    UpdateDate(date_picker::Date),
    CloseDate,
}

impl Sales {
    pub fn new(from: Date<Local>, to: Date<Local>, sales: Vec<(Item, Payment)>) -> Self {
        Self {
            from,
            to,
            receipts: sales.into_iter().fold(
                IndexMap::<_, Receipt<Event>, _>::new(),
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
            ),
        }
    }
}

impl Component<Message, Renderer> for Sales {
    type State = Option<Picker>;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Save => {
                let from = self.from;
                let to = self.to;
                let stats = self.receipts.clone();
                //Always return error to give info via modal
                return Some(
                    Sideffect::new(|| async move {
                        if !stats.is_empty() {
                            let path = save::save(stats, (from, to)).await?;
                            Ok(Message::OpenModal {
                                title: "Sparad",
                                content: format!("Sparad till {}", path.to_string_lossy()),
                            })
                        } else {
                            Err(Error::Other("Ingen försäljning att spara".into()))
                        }
                    })
                    .into(),
                );
            }
            Event::OpenDate(p) => {
                *state = Some(p);
                return None;
            }
            Event::UpdateDate(d) => {
                let date = Local.from_local_date(&d.into()).unwrap();
                match state {
                    Some(Picker::From) => {
                        self.from = date;
                    }
                    Some(Picker::To) => {
                        self.to = date;
                    }
                    None => (),
                };
                *state = None;
            }
            Event::CloseDate => {
                *state = None;
            }
        }

        let from = self.from;
        let to = self.to;
        Some(
            Sideffect::new(|| async move {
                Tab::Sales {
                    from,
                    to,
                    data: vec![],
                }
                .load()
                .await
            })
            .into(),
        )
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event> {
        DatePicker::new(
            state.is_some(),
            self.from.naive_local(),
            row![
                #nopad
                if !self.receipts.is_empty() {
                    Row::with_children(
                        self.receipts
                            .iter()
                            .map(|(payment, rec)| {
                                Container::new(
                                    column![
                                        #nopad
                                        BIG_TEXT::new(String::from(*payment)),
                                        Space::new(
                                            Length::Fill,
                                            Length::Units(SMALL_TEXT::size()),
                                        ),
                                        rec.clone(),
                                    ]
                                    .width(Length::Units(RECEIPT_WIDTH))
                                    .padding(DEF_PADDING),
                                )
                                .style(theme::Container::Border)
                                .into()
                            })
                            .collect(),
                    )
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .padding(DEF_PADDING)
                    .spacing(DEF_PADDING)
                } else {
                    row![
                        Space::with_width(Length::Fill),
                        BIG_TEXT::new("Ingen försäljning än"),
                        Space::with_width(Length::Fill),
                    ]
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_items(Alignment::Center)
                },
                Rule::vertical(DEF_PADDING),
                column![
                    BIG_TEXT::new("Visa Försäljning"),
                    Space::with_height(Length::Fill),
                    Text::new("Fr.o.m."),
                    Button::new(Text::new(self.from.to_string()))
                        .padding(DEF_PADDING)
                        .style(theme::Container::Border)
                        .on_press(Event::OpenDate(Picker::From)),
                    Text::new("T.o.m."),
                    Button::new(Text::new(self.to.to_string()))
                        .padding(DEF_PADDING)
                        .style(theme::Container::Border)
                        .on_press(Event::OpenDate(Picker::To)),
                    Space::with_height(Length::Fill),
                    Button::new(BIG_TEXT::new("Exportera"))
                        .on_press(Event::Save)
                        .padding(DEF_PADDING)
                        .style(theme::Container::Border)
                        .width(Length::Fill),
                ]
                .width(Length::Units(RECEIPT_WIDTH)),
            ],
            Event::CloseDate,
            Event::UpdateDate,
        )
        .into()
    }
}

impl<'a> From<Sales> for Element<'a, Message> {
    fn from(sales: Sales) -> Self {
        iced_lazy::component(sales)
    }
}
