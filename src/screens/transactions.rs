use {
    super::Sideffect,
    crate::{
        icons::Icon,
        item::{kind::Sales, Item},
        payment::Payment,
        print,
        receipt::Receipt,
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{column, row, SquareButton},
        Element, Renderer,
    },
    chrono::{DateTime, Local},
    frost::clickable::Clickable,
    iced::{
        widget::{Container, Row, Rule, Space},
        Length,
    },
    iced_lazy::Component,
    indexmap::IndexMap,
};

pub struct Transactions<M> {
    receipts: IndexMap<DateTime<Local>, Receipt<Event>>,
    sideffect: Box<dyn Fn(Sideffect) -> M>,
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

impl<M> Transactions<M> {
    pub fn new<F>(receipts: Vec<(DateTime<Local>, Item<Sales>, Payment)>, sideffect: F) -> Self
    where
        F: Fn(Sideffect) -> M + 'static,
    {
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
            sideffect: Box::new(sideffect),
        }
    }
}

impl<M> Component<M, Renderer> for Transactions<M> {
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<M> {
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
                    return Some((self.sideffect)(Sideffect::new(|| async move {
                        print::print(&receipt, time).await
                    })));
                }
            }
            _ => (),
        }
        None
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event> {
        row![
            #nopad
            Container::new(row![
                #nopad
                Clickable::new(Icon::Left)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .on_press(Event::ScrollLeft),
                Row::with_children(
                    self.receipts
                        .iter()
                        .skip(state.offset * 3)
                        .take(3)
                        .map(|(t, rec)| {
                            Container::new(rec.clone().on_press(Event::Select(*t)))
                                .padding(DEF_PADDING)
                                .style(theme::Container::Border)
                                .into()
                        },)
                        .collect()
                )
                .spacing(DEF_PADDING)
                .padding(DEF_PADDING),
                Clickable::new(Icon::Right)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y()
                    .on_press(Event::ScrollRight),
            ])
            .center_x()
            .width(Length::Fill),
            Rule::vertical(DEF_PADDING),
            column![
                match state.selected {
                    Some((_, ref rec)) => Element::from(rec.clone()),
                    None => Space::new(Length::Units(RECEIPT_WIDTH), Length::Fill).into(),
                },
                row![
                    #nopad
                    SquareButton::icon(Icon::Cross).on_press(Event::Deselect),
                    Space::with_width(Length::Fill),
                    SquareButton::icon(Icon::Print).on_press(Event::Print),
                ]
                .spacing(DEF_PADDING)
            ]
            .width(Length::Units(RECEIPT_WIDTH)),
        ]
        .into()
    }
}

impl<'a, M> From<Transactions<M>> for Element<'a, M>
where
    M: 'a,
{
    fn from(transactions: Transactions<M>) -> Self {
        iced_lazy::component(transactions)
    }
}
