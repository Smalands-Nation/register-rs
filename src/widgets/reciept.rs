use {
    super::Clickable,
    crate::{
        payment::Payment,
        reciept,
        screens::transactions::Item,
        styles::{DEF_PADDING, RECIEPT_WIDTH},
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
pub struct Reciept<M> {
    scroll: scrollable::State,
    click: button::State,
    message: Option<M>,
    inner: reciept::Reciept,
}

impl<M> Reciept<M>
where
    M: Clone,
{
    pub fn new() -> Self {
        Self::new_from(IndexMap::new(), 0, Payment::Swish)
    }

    pub fn new_from(items: IndexMap<String, Item>, sum: i32, payment: Payment) -> Self {
        Self {
            scroll: scrollable::State::new(),
            click: button::State::new(),
            message: None,
            inner: reciept::Reciept::new_from(items, sum, payment),
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
                            Scrollable::new(&mut self.scroll).spacing(DEF_PADDING),
                            |col, item| col.push(item.view()),
                        )
                        .height(Length::Fill),
                )
                .push(Text::new(format!("Total: {}kr", self.inner.sum)))
                .width(Length::Units(RECIEPT_WIDTH))
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

impl<M> Deref for Reciept<M> {
    type Target = reciept::Reciept;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<M> DerefMut for Reciept<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
