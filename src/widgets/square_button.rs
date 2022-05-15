use {
    crate::styles::{BIG_TEXT, SQUARE_BUTTON},
    iced::{
        alignment::{Horizontal, Vertical},
        pure::widget::{Button, Text},
        Length,
    },
};

pub struct SquareButton;

impl SquareButton {
    pub fn new<'a, M>(txt: impl Into<Text>) -> Button<'a, M>
    where
        M: Clone,
    {
        Button::new(
            txt.into()
                .size(BIG_TEXT)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Units(SQUARE_BUTTON))
        .height(Length::Units(SQUARE_BUTTON))
    }
}
