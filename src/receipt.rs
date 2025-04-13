use {
    crate::{
        theme::{Container, DEF_PADDING, RECEIPT_WIDTH},
        widgets::column,
    },
    backend::receipts::Receipt as RawReceipt,
    iced::{
        widget::{scrollable, Button, Column, Component, Scrollable, Text},
        Element, Length,
    },
};

#[derive(Debug, Clone)]
pub struct Receipt<M> {
    receipt: RawReceipt,
    msg: Option<M>,
}

impl<M> Receipt<M>
where
    M: Clone + std::fmt::Debug,
{
    pub fn on_press(mut self, msg: M) -> Self {
        self.msg = Some(msg);
        self
    }
}

impl<M> From<RawReceipt> for Receipt<M> {
    fn from(items: RawReceipt) -> Self {
        Self {
            receipt: items,
            msg: None,
        }
    }
}

impl<'a, M> Component<M> for Receipt<M>
where
    M: Clone + std::fmt::Debug + 'a,
{
    type Event = bool;
    type State = ();

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        if event {
            self.msg.clone()
        } else {
            None
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        Button::new(
            column![
                Scrollable::new(
                    Column::with_children(self.receipt.iter().map(|(item, amount)| Element::from(
                        crate::item::component::Item::new(item.clone(), *amount)
                    )))
                    .spacing(DEF_PADDING),
                )
                .direction(scrollable::Direction::Vertical(
                    scrollable::Properties::new()
                ))
                .height(Length::Fill)
                .width(Length::Fill),
                Text::new(format!("Total: {}kr", self.receipt.sum())),
            ]
            .width(Length::Fixed(RECEIPT_WIDTH))
            .spacing(DEF_PADDING),
        )
        .padding(0)
        .style(Container::Empty)
        .height(Length::Fill)
        .on_press(true)
        .into()
    }
}

impl<'a, M> From<Receipt<M>> for Element<'a, M>
where
    M: Clone + std::fmt::Debug + 'a,
{
    fn from(value: Receipt<M>) -> Self {
        iced::widget::component(value)
    }
}
