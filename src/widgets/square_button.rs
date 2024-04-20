use {
    super::BIG_TEXT,
    crate::{
        icons::Icon,
        theme::{Container, SQUARE_BUTTON},
    },
    iced::{
        alignment::{Horizontal, Vertical},
        widget::Button,
        Length,
    },
    std::borrow::Cow,
};

pub struct SquareButton;

impl SquareButton {
    pub fn text<'a, M>(txt: impl Into<Cow<'a, str>>) -> Button<'a, M>
    where
        M: Clone,
    {
        Button::new(
            BIG_TEXT::new(txt)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .style(Container::Border)
        .width(Length::Fixed(SQUARE_BUTTON))
        .height(Length::Fixed(SQUARE_BUTTON))
    }

    pub fn icon<'a, M>(icon: Icon) -> Button<'a, M>
    where
        M: Clone + 'a,
    {
        Button::new(icon)
            .style(Container::Border)
            .width(Length::Fixed(SQUARE_BUTTON))
            .height(Length::Fixed(SQUARE_BUTTON))
    }
}
