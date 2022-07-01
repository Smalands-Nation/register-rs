use {
    crate::{
        styles::{BORDERED, DEF_PADDING, SMALL_PADDING},
        widgets::SMALL_TEXT,
    },
    frost::pure::Clickable,
    iced::{
        alignment::Horizontal,
        pure::{
            widget::{Column, Container, Row, Space, Text},
            Element,
        },
        Length,
    },
    serde_derive::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Item {
    pub name: String,
    pub price: i32,
    pub kind: ItemKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemKind {
    Special,
    Regular { num: i32 },
    InStock(bool),
}

impl Item {
    pub fn is_special(&self) -> bool {
        matches!(self.kind, ItemKind::Special)
    }

    pub fn has_amount(&self) -> Option<i32> {
        match self.kind {
            ItemKind::Regular { num } => Some(num),
            _ => None,
        }
    }

    pub fn set_amount(&mut self, num: i32) {
        self.kind = match self.kind {
            ItemKind::Regular { .. } => ItemKind::Regular { num },
            _ => unreachable!("set_amount"),
        }
    }

    pub fn price_total(&self) -> i32 {
        match self.kind {
            ItemKind::Regular { num } => num * self.price,
            ItemKind::Special => self.price,
            _ => unreachable!("price_total"),
        }
    }

    pub fn is_in_stock(&self) -> bool {
        match self.kind {
            ItemKind::InStock(a) => a,
            _ => unreachable!("is_in_stock"),
        }
    }

    pub fn in_stock(&mut self, a: bool) {
        self.kind = match self.kind {
            ItemKind::InStock(_) => ItemKind::InStock(a),
            _ => unreachable!("is_in_stock"),
        }
    }

    pub fn as_widget<M>(&self) -> ItemWidget<M> {
        ItemWidget {
            msg: None,
            extra: None,
            inner: self.clone(),
        }
    }
}

impl std::cmp::PartialEq for Item {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name && (self.price == rhs.price || self.is_special())
    }
}

impl std::hash::Hash for Item {
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
impl std::ops::Add<Item> for Item {
    type Output = Self;
    fn add(self, rhs: Item) -> Self::Output {
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
                (ItemKind::Special, ItemKind::Special) => ItemKind::Special,
                (ItemKind::Regular { num }, ItemKind::Regular { num: num2 }) => {
                    ItemKind::Regular { num: num + num2 }
                }
                _ => unreachable!(),
            },
        }
    }
}

impl std::ops::AddAssign<Item> for Item {
    fn add_assign(&mut self, rhs: Item) {
        *self = self.clone() + rhs
    }
}

pub struct ItemWidget<'a, M> {
    msg: Option<M>,
    extra: Option<Element<'a, M>>,
    inner: Item,
}

impl<'a, M> ItemWidget<'a, M>
where
    M: Clone + 'a,
{
    pub fn on_press<F>(mut self, msg: F) -> Self
    where
        F: Fn(Item) -> M,
    {
        self.msg = Some(msg(self.inner.clone()));
        self
    }

    pub fn extra(mut self, extra: impl Into<Element<'a, M>>) -> Self {
        self.extra = Some(extra.into());
        self
    }
}

impl<'a, M> From<ItemWidget<'a, M>> for Element<'a, M>
where
    M: Clone + 'a,
{
    fn from(i: ItemWidget<'a, M>) -> Self {
        let body = Clickable(
            Container::new(
                Column::new()
                    .spacing(SMALL_PADDING)
                    .push(Text::new(i.inner.name.as_str()))
                    .push(match i.inner.kind {
                        ItemKind::Regular { num: 0 } | ItemKind::InStock(_) | ItemKind::Special => {
                            Row::new().push(
                                SMALL_TEXT::new(format!("{} kr", i.inner.price))
                                    .width(Length::Fill)
                                    .horizontal_alignment(Horizontal::Left),
                            )
                        }
                        ItemKind::Regular { num } => Row::new()
                            .push(SMALL_TEXT::new(format!("{}x{} kr", num, i.inner.price)))
                            .push(
                                SMALL_TEXT::new(format!("{} kr", i.inner.price_total()))
                                    .width(Length::Fill)
                                    .horizontal_alignment(Horizontal::Right),
                            ),
                    })
                    .push(
                        i.extra
                            .unwrap_or_else(|| Space::with_height(Length::Shrink).into()),
                    ),
            )
            .padding(DEF_PADDING)
            .width(Length::Fill)
            .style(BORDERED),
        )
        .width(Length::Fill);
        match i.msg {
            Some(msg) => body.on_press(msg),
            None => body,
        }
        .into()
    }
}
