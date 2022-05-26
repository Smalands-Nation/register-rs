use {
    crate::{
        styles::{BORDERED, DEF_PADDING, SMALL_PADDING},
        widgets::Clickable,
    },
    iced::{
        pure::{
            widget::{Column, Container, Text},
            Element,
        },
        Length,
    },
    serde_derive::{Deserialize, Serialize},
};

pub trait State: std::fmt::Debug + Clone {
    //fn view<'a, M: 'a>(item: Item<Self>) -> Element<'a, M>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item<S: State> {
    pub name: String,
    pub price: i32,
    //pub num: Option<i32>, //None for special items
    pub state: S,
}

impl<S: State> Item<S> {
    pub fn map<T, F>(self, map: F) -> Item<T>
    where
        T: State,
        F: FnOnce(S) -> T,
    {
        Item {
            name: self.name,
            price: self.price,
            state: map(self.state),
        }
    }

    pub fn as_widget<'a, M>(&self) -> ItemWidget<M, S>
    where
        Item<S>: Into<Element<'a, M>>,
    {
        ItemWidget {
            msg: None,
            inner: self.clone(),
        }
    }
}

pub struct ItemWidget<M, S: State> {
    msg: Option<M>,
    inner: Item<S>,
}

impl<'a, M, S> ItemWidget<M, S>
where
    M: Clone + 'a,
    S: State,
{
    pub fn on_press<F>(mut self, msg: F) -> Self
    where
        F: Fn(Item<S>) -> M,
    {
        self.msg = Some(msg(self.inner.clone()));
        self
    }
}

impl<'a, M, S> From<ItemWidget<M, S>> for Element<'a, M>
where
    M: Clone + 'a,
    S: State,
    Item<S>: Into<Element<'a, M>>,
{
    fn from(i: ItemWidget<M, S>) -> Self {
        let body = Clickable::new(
            Container::new(
                Column::new()
                    .spacing(SMALL_PADDING)
                    .push(Text::new(i.inner.name.as_str()))
                    .push(i.inner.into()),
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
