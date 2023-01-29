use {
    super::Sideffect,
    crate::{
        icons::Icon,
        item::{kind::Sales, Item},
        payment::Payment,
        print,
        receipt::Receipt,
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{calc::Calc, column, row, Grid, SquareButton, BIG_TEXT},
        Element, Renderer,
    },
    chrono::Local,
    iced::{
        widget::{Button, Checkbox, Container, Rule, Scrollable, Space},
        Alignment, Length,
    },
    iced_lazy::Component,
    rusqlite::params,
};

pub struct Menu<S, M> {
    menu: Vec<Item<Sales>>,
    sideffect: Box<dyn Fn(Sideffect<S>) -> M>,
}

#[derive(Clone)]
pub struct State {
    multiplier: u32,
    receipt: Receipt<Event>,
    print: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            multiplier: 1,
            receipt: Receipt::new(Payment::Swish),
            print: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Multiplier(u32),
    SellItem(Item<Sales>),
    ClearReceipt,
    TogglePrint(bool),
    Sell(Payment),
}

impl<S, M> Menu<S, M> {
    pub fn new<F>(menu: Vec<Item<Sales>>, sideffect: F) -> Self
    where
        F: Fn(Sideffect<S>) -> M + 'static,
    {
        Self {
            menu,
            sideffect: Box::new(sideffect),
        }
    }
}

impl<S, M> Component<M, Renderer> for Menu<S, M>
where
    S: Into<M> + Default + Clone,
{
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<M> {
        match event {
            Event::Multiplier(m) => {
                state.multiplier = m;
            }
            Event::ClearReceipt => {
                state.receipt = Receipt::new(Payment::Swish);
            }
            Event::SellItem(mut i) => {
                if let Some(0) = i.has_amount() {
                    i.set_amount(state.multiplier as i32);
                }
                state.receipt.add(i);
                state.multiplier = 1;
            }
            Event::TogglePrint(b) => state.print = b,
            Event::Sell(p) => {
                if !state.receipt.is_empty() {
                    let mut receipt = Receipt::new(Payment::Swish);
                    std::mem::swap(&mut receipt, &mut state.receipt);
                    let should_print = state.print;
                    return Some((self.sideffect)(Sideffect::new(|| async move {
                        let time = Local::now();
                        if should_print {
                            print::print(&receipt, time).await?;
                        }

                        let con = crate::DB.lock().await;

                        con.execute(
                            "INSERT INTO receipts (time, method) VALUES (?1, ?2)",
                            params![time, String::from(p)],
                        )?;

                        let mut stmt = con.prepare(
                            "INSERT INTO receipt_item (receipt, item, amount, price) \
                                            VALUES (?1, ?2, ?3, ?4)",
                        )?;

                        for item in receipt.items.iter() {
                            stmt.execute(params![
                                time,
                                item.name,
                                item.has_amount().unwrap_or(0), //Special item has no ammount
                                item.price,
                            ])?;
                        }

                        Ok(S::default())
                    })));
                }
            }
        };
        None
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event> {
        let State {
            multiplier,
            receipt,
            print,
        } = state.clone();
        row![
            #nopad
            Container::new(Calc::new(multiplier ,Event::Multiplier))
                .padding(DEF_PADDING)
                .center_x()
                .center_y()
                .width(Length::Units(RECEIPT_WIDTH))
                .height(Length::Fill),
            Rule::vertical(DEF_PADDING),
            Scrollable::new(
                Grid::with_children(
                    self.menu.len() as u32 / 3,
                    3,
                    self.menu
                        .iter()
                        .map(|i| i.as_widget(true).on_press(Event::SellItem).into())
                        .collect(),
                )
                .width(Length::Fill)
                .spacing(DEF_PADDING)
                .padding(DEF_PADDING),
            ),
            Rule::vertical(DEF_PADDING),
            column![
                row![
                    #nopad
                    BIG_TEXT::new("Kvitto"),
                    Space::with_width(Length::Fill),
                    SquareButton::icon(Icon::Cross).on_press(Event::ClearReceipt),
                ]
                .align_items(Alignment::Center),
                receipt,
                Checkbox::new(print, "Printa kvitto", Event::TogglePrint),
                row![
                    #nopad
                    Button::new(Payment::Swish)
                        .on_press(Event::Sell(Payment::Swish))
                        .padding(DEF_PADDING)
                        .style(theme::Container::Border)
                        .width(Length::Fill),
                    Button::new(Payment::Paypal)
                        .on_press(Event::Sell(Payment::Paypal))
                        .padding(DEF_PADDING)
                        .style(theme::Container::Border)
                        .width(Length::Fill),
                ]
                .spacing(DEF_PADDING)
            ]
            .width(Length::Units(RECEIPT_WIDTH)),
        ]
        .into()
    }
}

impl<'a, S, M> From<Menu<S, M>> for Element<'a, M>
where
    S: Into<M> + Default + Clone + 'a,
    M: 'a,
{
    fn from(menu: Menu<S, M>) -> Self {
        iced_lazy::component(menu)
    }
}
