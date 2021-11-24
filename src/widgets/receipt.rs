use {
    super::Clickable,
    crate::{
        payment::Payment,
        receipt,
        screens::transactions::Item,
        styles::{DEF_PADDING, RECEIPT_WIDTH},
    },
    core::ops::{Deref, DerefMut},
    iced::{
        button,
        scrollable::{self, Scrollable},
        Column, Element, Length, Text,
    },
    indexmap::IndexMap,
};

#[derive(Debug, Clone)]
pub struct Receipt<M> {
    scroll: scrollable::State,
    click: button::State,
    message: Option<M>,
    inner: receipt::Receipt,
}

impl<M> Receipt<M>
where
    M: Clone,
{
    pub fn new(payment: Payment) -> Self {
        Self::new_from(IndexMap::new(), 0, payment)
    }

    pub fn new_from(items: IndexMap<String, Item>, sum: i32, payment: Payment) -> Self {
        Self {
            scroll: scrollable::State::new(),
            click: button::State::new(),
            message: None,
            inner: receipt::Receipt::new_from(items, sum, payment),
        }
    }

    pub fn on_press(mut self, msg: M) -> Self {
        self.message = Some(msg);
        self
    }

    pub fn view(&mut self) -> Element<M> {
        let body = Clickable::new(
            &mut self.click,
            Column::new()
                .push(
                    self.inner
                        .items
                        .values_mut()
                        .fold(
                            Scrollable::new(&mut self.scroll)
                                .spacing(DEF_PADDING)
                                .scrollbar_width(10),
                            |col, item| col.push(item.view()),
                        )
                        .height(Length::Fill),
                )
                .push(Text::new(format!("Total: {}kr", self.inner.sum)))
                .width(Length::Units(RECEIPT_WIDTH))
                .spacing(DEF_PADDING),
        )
        .padding(0)
        .height(Length::Fill);
        match &self.message {
            Some(msg) => body.on_press(msg.clone()),
            None => body,
        }
        .into()
    }
}

impl<M> Deref for Receipt<M> {
    type Target = receipt::Receipt;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<M> DerefMut for Receipt<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
