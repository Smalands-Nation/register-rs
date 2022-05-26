use {
    crate::{
        item,
        payment::Payment,
        styles::{DEF_PADDING, RECEIPT_WIDTH, SMALL_TEXT},
        widgets::Clickable,
    },
    iced::{
        alignment::Horizontal,
        pure::{
            widget::{Column, Row, Scrollable, Text},
            Element,
        },
        Length,
    },
    indexmap::IndexSet,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReceiptItem {
    Regular { num: i32 },
    Special,
}
impl item::State for ReceiptItem {}
impl<'a, M: 'a> From<Item> for Element<'a, M> {
    fn from(item: Item) -> Element<'a, M> {
        match item.state {
            Special => Row::new().push(
                Text::new(format!("{} kr", item.price_total()))
                    .size(SMALL_TEXT)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Right),
            ),
            Regular { num } => Row::new()
                .push(Text::new(format!("{}x{} kr", num, item.price)).size(SMALL_TEXT))
                .push(
                    Text::new(format!("{} kr", item.price_total()))
                        .size(SMALL_TEXT)
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Right),
                ),
        }
        .into()
    }
}
use ReceiptItem::*;
pub type Item = item::Item<ReceiptItem>;
impl Item {
    pub fn price_total(&self) -> i32 {
        match self.state {
            Regular { num } => num * self.price,
            Special => self.price,
        }
    }
}

impl std::cmp::Eq for Item {}
impl std::cmp::PartialEq for Item {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
            && match self.state {
                Regular { .. } => self.price == rhs.price,
                Special => true,
            }
    }
}

impl std::hash::Hash for Item {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.name.hash(state);
        if let Regular { .. } = self.state {
            self.price.hash(state);
        }
    }
}

///Adds two of the same item into one with their combined amounts
impl std::ops::Add<Item> for Item {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self != rhs {
            unreachable!("Tried to add different items:\n'{:#?}'\n'{:#?}'", self, rhs);
        }
        match self.state {
            Special => Self {
                name: self.name.clone(),
                price: self.price + rhs.price,
                state: self.state,
            },
            Regular { num } => Self {
                name: self.name.clone(),
                price: self.price,
                state: if let Regular { num: num2 } = rhs.state {
                    Regular { num: num + num2 }
                } else {
                    unreachable!()
                },
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Receipt {
    pub items: IndexSet<Item>,
    pub sum: i32,
    pub payment: Payment,
}

impl Receipt {
    pub fn new(payment: Payment) -> Self {
        Self::new_from(IndexSet::new(), 0, payment)
    }

    pub fn new_from(items: IndexSet<Item>, sum: i32, payment: Payment) -> Self {
        Self {
            items,
            sum,
            payment,
        }
    }

    pub fn add(&mut self, item: Item) {
        self.sum += item.price_total();
        let it = self.items.get(&item).cloned();
        match it {
            Some(it) => {
                self.items.replace(it + item);
            }
            None => {
                self.items.insert(item);
            }
        }
        self.items.sort_by(|v1, v2| match (v1.state, v2.state) {
            (Regular { .. }, Regular { .. }) | (Special, Special) => std::cmp::Ordering::Equal,
            (Regular { .. }, Special) => std::cmp::Ordering::Less,
            (Special, Regular { .. }) => std::cmp::Ordering::Greater,
        });
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn as_widget<M>(&mut self) -> ReceiptWidget<M> {
        ReceiptWidget {
            message: None,
            inner: self,
        }
    }
}

#[derive(Debug)]
pub struct ReceiptWidget<'a, M> {
    message: Option<M>,
    inner: &'a mut Receipt,
}

impl<'a, M> ReceiptWidget<'a, M>
where
    M: Clone + 'a,
{
    pub fn on_press(mut self, msg: M) -> Self {
        self.message = Some(msg);
        self
    }
}

impl<'a, M> From<ReceiptWidget<'a, M>> for Element<'a, M>
where
    M: Clone + 'a,
{
    fn from(r: ReceiptWidget<'a, M>) -> Self {
        let body = Clickable::new(
            Column::new()
                .push(
                    Scrollable::new(
                        r.inner
                            .items
                            .iter()
                            .fold(Column::new().spacing(DEF_PADDING), |col, item| {
                                col.push(item.as_widget())
                            }),
                    )
                    .scrollbar_width(10)
                    .height(Length::Fill),
                )
                .push(Text::new(format!("Total: {}kr", r.inner.sum)))
                .width(Length::Units(RECEIPT_WIDTH))
                .spacing(DEF_PADDING),
        )
        .padding(0)
        .height(Length::Fill);
        match &r.message {
            Some(msg) => body.on_press(msg.clone()),
            None => body,
        }
        .into()
    }
}
