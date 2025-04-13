use {
    super::{Message, Sideffect},
    crate::{
        icons::Icon,
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{padded_column, row, SquareButton},
    },
    backend::receipts::Receipt,
    chrono::{DateTime, Local},
    iced::{
        widget::{scrollable::Direction, Component, Container, Row, Rule, Scrollable, Space},
        Element, Length,
    },
    indexmap::IndexMap,
};

pub struct Transactions {
    receipts: IndexMap<DateTime<Local>, Receipt>,
}

#[derive(Default)]
pub struct State {
    selected: Option<Receipt>,
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
    pub fn new(receipts: IndexMap<DateTime<Local>, Receipt>) -> Self {
        Self { receipts }
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
                state.selected = self.receipts.get(&time).cloned();
            }
            Event::Deselect => state.selected = None,
            Event::Print => {
                if let Some(receipt) = state.selected.take() {
                    return Some(
                        Sideffect::new(|| async move {
                            receipt.print().await?;
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
                    Container::new(
                        crate::receipt::Receipt::from(rec.clone()).on_press(Event::Select(*t)),
                    )
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
                    Some(ref rec) => Element::from(crate::receipt::Receipt::from(rec.clone())),
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
