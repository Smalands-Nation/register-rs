use {
    super::BIG_TEXT,
    crate::{icons::Icon, styles::SQUARE_BUTTON},
    iced::{
        alignment::{Horizontal, Vertical},
        widget::Button,
        Length,
    },
    std::borrow::Cow,
};

pub struct SquareButton;

impl SquareButton {
    pub fn text<'a, M, R>(txt: impl Into<Cow<'a, str>>) -> Button<'a, M, R>
    where
        M: Clone,
        R: iced_native::Renderer + iced_native::text::Renderer + 'a,
        R::Theme: iced_native::widget::button::StyleSheet + iced_native::widget::text::StyleSheet,
    {
        Button::new(
            BIG_TEXT::new(txt)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Units(SQUARE_BUTTON))
        .height(Length::Units(SQUARE_BUTTON))
    }

    pub fn icon<'a, M, R>(icon: Icon) -> Button<'a, M, R>
    where
        M: Clone + 'a,
        R: iced_native::Renderer + iced_native::text::Renderer + 'a,
        R::Theme: iced_native::widget::button::StyleSheet + iced_native::widget::text::StyleSheet,
        iced::Font: Into<R::Font>,
    {
        Button::new(icon)
            .width(Length::Units(SQUARE_BUTTON))
            .height(Length::Units(SQUARE_BUTTON))
    }
}
