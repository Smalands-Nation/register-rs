use {
    crate::{
        styles::{BORDERED, DEF_PADDING, SMALL_PADDING},
        widgets::SMALL_TEXT,
    },
    frost::pure::Clickable,
    iced::{
        alignment::Horizontal,
        pure::{
            widget::{Checkbox, Column, Container, Row, Text},
            Element,
        },
        Length,
    },
    kind::*,
};

pub mod kind {
    pub trait ItemKind: Clone {}

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Sales {
        Special,
        Regular { num: i32 },
    }
    impl ItemKind for Sales {}

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Stock {
        pub idx: usize,
        pub available: bool,
    }
    impl ItemKind for Stock {}
}

#[derive(Debug, Clone)]
pub struct Item<K: ItemKind> {
    pub name: String,
    pub price: i32,
    pub kind: K,
}

//#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]

impl Item<Sales> {
    pub fn is_special(&self) -> bool {
        matches!(self.kind, Sales::Special)
    }

    pub fn has_amount(&self) -> Option<i32> {
        match self.kind {
            Sales::Regular { num } => Some(num),
            _ => None,
        }
    }

    pub fn set_amount(&mut self, num: i32) {
        self.kind = match self.kind {
            Sales::Regular { .. } => Sales::Regular { num },
            _ => unreachable!("set_amount on Sales::Special"),
        }
    }

    pub fn price_total(&self) -> i32 {
        match self.kind {
            Sales::Regular { num } => num * self.price,
            Sales::Special => self.price,
        }
    }
}

impl Item<Stock> {
    pub fn with_index(mut self, idx: usize) -> Self {
        self.kind.idx = idx;
        self
    }

    pub fn is_in_stock(&self) -> bool {
        self.kind.available
    }

    pub fn in_stock(&mut self, a: bool) {
        self.kind.available = a;
    }
}

impl<K: ItemKind> Item<K> {
    pub fn as_widget<M>(&self) -> ItemWidget<M, K> {
        ItemWidget {
            msg: None,
            inner: self.clone(),
        }
    }
}

impl std::cmp::PartialEq for Item<Sales> {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name && (self.price == rhs.price || self.is_special())
    }
}

impl std::cmp::Eq for Item<Sales> {}

impl std::hash::Hash for Item<Sales> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.name.hash(state);
        if !self.is_special() {
            self.price.hash(state);
        }
    }
}

///Adds two of the same item into one with their combined amounts
impl std::ops::Add for Item<Sales> {
    type Output = Self;
    fn add(self, rhs: Item<Sales>) -> Self::Output {
        if self != rhs {
            unreachable!("Tried to add different items:\n'{:#?}'\n'{:#?}'", self, rhs);
        }
        Self {
            name: self.name.clone(),
            price: if self.is_special() {
                self.price + rhs.price
            } else {
                self.price
            },
            kind: match (self.kind, rhs.kind) {
                (Sales::Special, Sales::Special) => Sales::Special,
                (Sales::Regular { num }, Sales::Regular { num: num2 }) => {
                    Sales::Regular { num: num + num2 }
                }
                _ => unreachable!(),
            },
        }
    }
}

impl std::ops::AddAssign for Item<Sales> {
    fn add_assign(&mut self, rhs: Item<Sales>) {
        *self = self.clone() + rhs
    }
}

pub struct ItemWidget<M, K: ItemKind> {
    msg: Option<M>,
    inner: Item<K>,
}

impl<'a, M, K> ItemWidget<M, K>
where
    M: Clone + 'a,
    K: ItemKind,
{
    pub fn on_press<F>(mut self, msg: F) -> Self
    where
        F: Fn(Item<K>) -> M,
    {
        self.msg = Some(msg(self.inner.clone()));
        self
    }

    fn element(&self, inner: Vec<Element<'a, M>>) -> Element<'a, M> {
        let body = Clickable(
            Container::new(
                Column::with_children(
                    vec![Text::new(self.inner.name.as_str()).into()]
                        .into_iter()
                        .chain(inner)
                        .collect(),
                )
                .spacing(SMALL_PADDING),
            )
            .padding(DEF_PADDING)
            .width(Length::Fill)
            .style(BORDERED),
        )
        .width(Length::Fill);
        match self.msg {
            Some(ref msg) => body.on_press(msg.clone()),
            None => body,
        }
        .into()
    }
}

impl<'a, M> From<ItemWidget<M, Sales>> for Element<'a, M>
where
    M: Clone + 'a,
{
    fn from(i: ItemWidget<M, Sales>) -> Self {
        i.element(vec![match i.inner.kind {
            Sales::Regular { num: 0 } | Sales::Special => Row::new().push(
                SMALL_TEXT::new(format!("{} kr", i.inner.price))
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left),
            ),
            Sales::Regular { num } => Row::new()
                .push(SMALL_TEXT::new(format!("{}x{} kr", num, i.inner.price)))
                .push(
                    SMALL_TEXT::new(format!("{} kr", i.inner.price_total()))
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Right),
                ),
        }
        .into()])
    }
}

use crate::screens::manager::Message;
impl From<ItemWidget<Message, Stock>> for Element<'_, Message> {
    fn from(i: ItemWidget<Message, Stock>) -> Self {
        i.element(vec![
            Row::new()
                .push(
                    SMALL_TEXT::new(format!("{} kr", i.inner.price))
                        .width(Length::Fill)
                        .horizontal_alignment(Horizontal::Left),
                )
                .into(),
            Checkbox::new(i.inner.kind.available, "I Lager", move |b| {
                Message::ToggleItem(i.inner.kind.idx, b)
            })
            .into(),
        ])
    }
}
