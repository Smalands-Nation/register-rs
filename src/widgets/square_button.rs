use {
    super::BIG_TEXT,
    crate::{icons::Icon, styles::SQUARE_BUTTON},
    iced::{
        alignment::{Horizontal, Vertical},
        pure::widget::Button,
        Length,
    },
};

pub struct SquareButton;

impl SquareButton {
    pub fn text<'a, M>(txt: impl Into<String>) -> Button<'a, M>
    where
        M: Clone,
    {
        Button::new(
            BIG_TEXT::new(txt)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Units(SQUARE_BUTTON))
        .height(Length::Units(SQUARE_BUTTON))
    }

    pub fn icon<'a, M>(icon: Icon) -> Button<'a, M>
    where
        M: Clone,
    {
        Button::new(icon)
            .width(Length::Units(SQUARE_BUTTON))
            .height(Length::Units(SQUARE_BUTTON))
    }
}
