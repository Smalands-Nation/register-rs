use {
    crate::{
        styles::{BORDERED, DEF_PADDING, SMALL_PADDING, SMALL_TEXT},
        widgets::Clickable,
    },
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub name: String,
    pub price: i32,
    pub num: Option<i32>, //None for special items
}

impl Item {
    pub fn special(&self) -> bool {
        self.num.is_none()
    }

    pub fn price_total(&self) -> i32 {
        match self.num {
            Some(n) => n * self.price,
            None => self.price,
        }
    }

    pub fn as_widget<M>(&mut self) -> ItemWidget<M> {
        ItemWidget {
            msg: None,
            extra: None,
            inner: self,
        }
    }
}

///Adds two of the same item into one with their combined amounts
impl std::ops::Add<Item> for Item {
    type Output = Self;
    fn add(self, rhs: Item) -> Self::Output {
        if self.name != rhs.name {
            unreachable!("Tried to add different items");
        }
        Self {
            /*
            name: self.name,
            price: if self.special() {
                self.price + rhs.price
            } else {
                self.price
            },
            num: (|| -> Option<i32> { Some(self.num? + rhs.num?) })(),
            */
            name: String::new(),
            price: 0,
            num: None,
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
    inner: &'a mut Item,
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
        let body = Clickable::new(
            Container::new(
                Column::new()
                    .spacing(SMALL_PADDING)
                    .push(Text::new(i.inner.name.as_str()))
                    .push(match i.inner.num {
                        None | Some(0) => Row::new().push(
                            Text::new(format!("{} kr", i.inner.price))
                                .size(SMALL_TEXT)
                                .width(Length::Fill)
                                .horizontal_alignment(Horizontal::Right),
                        ),
                        Some(n) => Row::new()
                            .push(Text::new(format!("{}x{} kr", n, i.inner.price)).size(SMALL_TEXT))
                            .push(
                                Text::new(format!("{} kr", i.inner.price_total()))
                                    .size(SMALL_TEXT)
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
