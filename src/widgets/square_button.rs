use {
    crate::styles::{BIG_TEXT, SQUARE_BUTTON},
    iced::{
        button::{self, Button},
        HorizontalAlignment, Text, VerticalAlignment,
    },
};

pub struct SquareButton;

impl SquareButton {
pub fn new<M>(state: &mut button::State, txt: Text) -> Button<M>
where
    M: Clone,
{
    Button::new(
        state,
        txt.size(BIG_TEXT)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center),
    )
    .min_width(SQUARE_BUTTON as u32)
    .min_height(SQUARE_BUTTON as u32)
}}
