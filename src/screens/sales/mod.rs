use {
    super::{Message, Sideffect, TabId},
    crate::{
        error::Error,
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{column, padded_column, padded_row, row, BIG_TEXT, SMALL_TEXT},
    },
    backend::summary::Summary,
    chrono::NaiveDate,
    iced::{
        widget::{Button, Component, Container, Row, Rule, Space, Text},
        Alignment, Element, Length,
    },
    iced_aw::date_picker::{self, DatePicker},
};

#[derive(Debug, Clone)]
pub enum Picker {
    From,
    To,
}

pub struct Sales {
    from: NaiveDate,
    to: NaiveDate,
    summary: Summary,
}

#[derive(Debug, Clone)]
pub enum Event {
    Save,
    OpenDate(Picker),
    UpdateDate(date_picker::Date),
    CloseDate,
}

impl Sales {
    pub fn new(summary: Summary) -> Self {
        let from = summary.from().naive_local().date();
        let to = summary.to().naive_local().date();
        Self { from, to, summary }
    }
}

impl Component<Message> for Sales {
    type State = Option<Picker>;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Save => {
                let summary = self.summary.clone();
                //Always return error to give info via modal
                return Some(
                    Sideffect::new(|| async move {
                        if !summary.is_empty() {
                            let path = summary.save().await?;
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
                let date = d.into();
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
        Some(Sideffect::new(|| async move { TabId::Sales { from, to }.load().await }).into())
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event> {
        row![
            if !self.summary.is_empty() {
                Row::with_children(self.summary.receipts().map(|(payment, rec)| {
                    Container::new(
                        column![
                            BIG_TEXT::new(payment.to_string()),
                            Space::new(Length::Fill, Length::Fixed(SMALL_TEXT::size() as f32)),
                            crate::receipt::Receipt::from(rec.clone()),
                        ]
                        .width(Length::Fixed(RECEIPT_WIDTH))
                        .padding(DEF_PADDING),
                    )
                    .style(theme::Container::Border)
                    .into()
                }))
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .padding(DEF_PADDING)
                .spacing(DEF_PADDING)
            } else {
                padded_row![
                    Space::with_width(Length::Fill),
                    BIG_TEXT::new("Ingen försäljning än"),
                    Space::with_width(Length::Fill),
                ]
                .width(Length::Fill)
                .height(Length::Fill)
                .align_items(Alignment::Center)
            },
            Rule::vertical(DEF_PADDING),
            padded_column![
                BIG_TEXT::new("Visa Försäljning"),
                Space::with_height(Length::Fill),
                Text::new("Fr.o.m."),
                DatePicker::new(
                    matches!(state, Some(Picker::From)),
                    self.from,
                    Button::new(Text::new(self.from.format("%F").to_string()))
                        .width(Length::Fill)
                        .padding(DEF_PADDING)
                        .style(theme::Container::Border)
                        .on_press(Event::OpenDate(Picker::From)),
                    Event::CloseDate,
                    Event::UpdateDate,
                )
                .font_size(SMALL_TEXT::size()),
                Text::new("T.o.m."),
                DatePicker::new(
                    matches!(state, Some(Picker::To)),
                    self.from,
                    Button::new(Text::new(self.to.format("%F").to_string()))
                        .width(Length::Fill)
                        .padding(DEF_PADDING)
                        .style(theme::Container::Border)
                        .on_press(Event::OpenDate(Picker::To)),
                    Event::CloseDate,
                    Event::UpdateDate,
                )
                .font_size(SMALL_TEXT::size()),
                Space::with_height(Length::Fill),
                Button::new(BIG_TEXT::new("Exportera"))
                    .on_press(Event::Save)
                    .padding(DEF_PADDING)
                    .style(theme::Container::Border)
                    .width(Length::Fill),
            ]
            .width(Length::Fixed(RECEIPT_WIDTH)),
        ]
        .into()
    }
}

impl<'a> From<Sales> for Element<'a, Message> {
    fn from(sales: Sales) -> Self {
        iced::widget::component(sales)
    }
}
