use {
    super::{Message, Sideffect},
    crate::{
        icons::Icon,
        theme::{self, DEF_PADDING, RECEIPT_WIDTH},
        widgets::{calc::Calc, padded_column, row, SquareButton, BIG_TEXT},
    },
    backend::{
        items::Item,
        receipts::{Payment, Receipt},
    },
    chrono::Local,
    iced::{
        widget::{
            image::{Handle, Image},
            Button, Checkbox, Component, Container, Responsive, Rule, Scrollable, Space,
        },
        Alignment, Element, Length, Size,
    },
    iced_aw::Wrap,
};

pub struct Menu {
    menu: Vec<Item>,
}

#[derive(Clone)]
pub struct State {
    multiplier: u32,
    receipt: Receipt,
    print: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            multiplier: 1,
            receipt: Receipt::default(),
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
                state.receipt = Receipt::default();
            }
            Event::SellItem(i) => {
                let item = self.menu[i].clone();
                state.receipt.insert(item, state.multiplier as i32);
                state.multiplier = 1;
            }
            Event::TogglePrint(b) => state.print = b,
            Event::Sell(p) => {
                if !state.receipt.is_empty() {
                    let receipt = std::mem::take(&mut state.receipt)
                        .with_payment(p)
                        .with_time(Local::now());
                    let should_print = state.print;
                    return Some(
                        Sideffect::new(|| async move {
                            if should_print {
                                receipt.print().await?;
                            }

                            receipt.insert_sale().await?;
                            Ok(Message::None)
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
                                crate::item::component::Item::from(item)
                                    .on_press(Event::SellItem(i))
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
                crate::receipt::Receipt::from(receipt),
                Checkbox::new("Printa kvitto", print)
                    .text_size(30)
                    .width(Length::Fill)
                    .on_toggle(Event::TogglePrint),
                row![
                    payment_to_button(Payment::Swish),
                    payment_to_button(Payment::Paypal)
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

fn payment_to_button<'a>(p: Payment) -> Button<'a, Event> {
    let image = Image::new(Handle::from_memory(match p {
        Payment::Swish => include_bytes!("../../resources/swish.png").to_vec(),
        Payment::Paypal => include_bytes!("../../resources/paypal.png").to_vec(),
        _ => unreachable!("Payment to Image"),
    }));

    Button::new(image)
        .on_press(Event::Sell(p))
        .padding(DEF_PADDING)
        .style(theme::Container::Border)
        .width(Length::Fill)
}
