use {
    super::BIG_TEXT,
    crate::{icons::Icon, styles::SQUARE_BUTTON, Renderer},
    iced::{
        alignment::{Horizontal, Vertical},
        widget::Button,
        Length,
    },
    std::borrow::Cow,
};

pub struct SquareButton;

impl SquareButton {
    pub fn text<'a, M>(txt: impl Into<Cow<'a, str>>) -> Button<'a, M, Renderer>
    where
        M: Clone,
    {
        Button::new(
            BIG_TEXT::new(txt)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .style(iced_native::theme::Button::Text)
        .width(Length::Units(SQUARE_BUTTON))
        .height(Length::Units(SQUARE_BUTTON))
    }

    pub fn icon<'a, M>(icon: Icon) -> Button<'a, M, Renderer>
    where
        M: Clone + 'a,
    {
        Button::new(icon)
            .style(iced_native::theme::Button::Text)
            .width(Length::Units(SQUARE_BUTTON))
            .height(Length::Units(SQUARE_BUTTON))
    }
}
