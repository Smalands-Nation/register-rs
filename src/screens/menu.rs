use {
    super::{Message, Sideffect},
    crate::{
        icons::Icon,
        item::Item,
        payment::Payment,
        print,
        receipt::Receipt,
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{calc::Calc, padded_column, row, SquareButton, BIG_TEXT},
    },
    chrono::Local,
    iced::{
        widget::{Button, Checkbox, Component, Container, Responsive, Rule, Scrollable, Space},
        Alignment, Element, Length, Size,
    },
    iced_aw::Wrap,
    rusqlite::params,
};

pub struct Menu {
    menu: Vec<Item>,
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
    SellItem(usize),
    ClearReceipt,
    TogglePrint(bool),
    Sell(Payment),
}

impl Menu {
    pub fn new(menu: Vec<Item>) -> Self {
        Self { menu }
    }
}

impl Component<Message> for Menu {
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Multiplier(m) => {
                state.multiplier = m;
            }
            Event::ClearReceipt => {
                state.receipt = Receipt::new(Payment::Swish);
            }
            Event::SellItem(i) => {
                let mut item = self.menu[i].clone();
                if let Some(0) = item.has_amount() {
                    item.set_amount(state.multiplier as i32);
                    state.receipt.add(item);
                } else if item.is_special() {
                    for _ in 0..state.multiplier {
                        state.receipt.add(item.clone());
                    }
                }
                state.multiplier = 1;
            }
            Event::TogglePrint(b) => state.print = b,
            Event::Sell(p) => {
                if !state.receipt.is_empty() {
                    let mut receipt = Receipt::new(Payment::Swish);
                    std::mem::swap(&mut receipt, &mut state.receipt);
                    let should_print = state.print;
                    return Some(
                        Sideffect::new(|| async move {
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

                            Ok(().into())
                        })
                        .into(),
                    );
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
            Container::new(Calc::new(multiplier, Event::Multiplier))
                .padding(DEF_PADDING)
                .center_x()
                .center_y()
                .width(Length::Fixed(RECEIPT_WIDTH))
                .height(Length::Fill),
            Rule::vertical(DEF_PADDING),
            Responsive::new(|Size { width, .. }| {
                Scrollable::new(
                    Wrap::with_elements(
                        self.menu
                            .iter()
                            .cloned()
                            .enumerate()
                            .map(|(i, item)| {
                                item.on_press(Event::SellItem(i))
                                    .width(Length::Fixed(width / 3.0 - 2.0 * DEF_PADDING as f32))
                                    .into()
                            })
                            .collect(),
                    )
                    .align_items(Alignment::End)
                    .spacing(DEF_PADDING as f32)
                    .line_spacing(DEF_PADDING as f32)
                    .padding(DEF_PADDING as f32),
                )
                .into()
            }),
            Rule::vertical(DEF_PADDING),
            padded_column![
                row![
                    BIG_TEXT::new("Kvitto"),
                    Space::with_width(Length::Fill),
                    SquareButton::icon(Icon::Cross).on_press(Event::ClearReceipt),
                ]
                .align_items(Alignment::Center),
                receipt,
                Checkbox::new("Printa kvitto", print)
                    .text_size(30)
                    .width(Length::Fill)
                    .on_toggle(Event::TogglePrint),
                row![
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
            .width(Length::Fixed(RECEIPT_WIDTH)),
        ]
        .into()
    }
}

impl<'a> From<Menu> for Element<'a, Message> {
    fn from(menu: Menu) -> Self {
        iced::widget::component(menu)
    }
}
