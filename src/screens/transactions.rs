use {
    super::{Message, Sideffect},
    crate::{
        icons::Icon,
        item::Item,
        payment::Payment,
        print,
        receipt::Receipt,
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{padded_column, row, SquareButton},
    },
    chrono::{DateTime, Local},
    iced::{
        widget::{scrollable::Direction, Component, Container, Row, Rule, Scrollable, Space},
        Element, Length,
    },
    indexmap::IndexMap,
};

pub struct Transactions {
    receipts: IndexMap<DateTime<Local>, Receipt<Event>>,
}

#[derive(Default)]
pub struct State {
    selected: Option<(DateTime<Local>, Receipt<Event>)>,
    offset: usize,
}

#[derive(Debug, Clone)]
pub enum Event {
    ScrollLeft,
    ScrollRight,
    Select(DateTime<Local>),
    Deselect,
    Print,
}

impl Transactions {
    pub fn new(receipts: Vec<(DateTime<Local>, Item, Payment)>) -> Self {
        Self {
            receipts: receipts.into_iter().fold(
                IndexMap::<_, Receipt<Event>, _>::new(),
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
            ),
        }
    }
}

impl Component<Message> for Transactions {
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::ScrollLeft if state.offset > 0 => state.offset -= 1,
            Event::ScrollRight
                if !self.receipts.is_empty() && state.offset < (self.receipts.len() - 1) / 3 =>
            {
                state.offset += 1
            }
            Event::Select(time) => {
                state.selected = self
                    .receipts
                    .get_key_value(&time)
                    .map(|(k, v)| (*k, v.clone()));
            }
            Event::Deselect => state.selected = None,
            Event::Print => {
                if let Some((time, receipt)) = &state.selected {
                    let receipt = receipt.clone();
                    let time = *time;
                    return Some(
                        Sideffect::new(|| async move {
                            print::print(&receipt, time).await?;
                            Ok(().into())
                        })
                        .into(),
                    );
                }
            }
            _ => (),
        }
        None
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event> {
        row![
            Scrollable::new(
                Row::with_children(self.receipts.iter().map(|(t, rec)| {
                    Container::new(rec.clone().on_press(Event::Select(*t)))
                        .padding(DEF_PADDING)
                        .style(theme::Container::Border)
                        .into()
                }))
                .spacing(DEF_PADDING)
                .padding(DEF_PADDING)
            )
            .direction(Direction::Horizontal(Default::default()))
            .width(Length::Fill),
            Rule::vertical(DEF_PADDING),
            padded_column![
                match state.selected {
                    Some((_, ref rec)) => Element::from(rec.clone()),
                    None => Space::new(Length::Fixed(RECEIPT_WIDTH), Length::Fill).into(),
                },
                row![
                    SquareButton::icon(Icon::Cross).on_press(Event::Deselect),
                    Space::with_width(Length::Fill),
                    SquareButton::icon(Icon::Print).on_press(Event::Print),
                ]
                .spacing(DEF_PADDING)
            ]
            .width(Length::Fixed(RECEIPT_WIDTH)),
        ]
        .into()
    }
}

impl<'a> From<Transactions> for Element<'a, Message> {
    fn from(transactions: Transactions) -> Self {
        iced::widget::component(transactions)
    }
}
