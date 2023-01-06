use {
    iced::{widget::container, Background, Color},
    iced_aw::style::tab_bar,
};

const BORDER_RADIUS: f32 = 2.0;
pub const BORDER_WIDTH: f32 = 2.0;

pub struct Bordered {
    background: Color,
}

impl Default for Bordered {
    fn default() -> Self {
        Bordered {
            background: Color::WHITE,
        }
    }
}

impl Bordered {
    pub fn new(background: Color) -> Self {
        Self { background }
    }
}

impl container::StyleSheet for Bordered {
    type Style = Self;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::BLACK),
            background: Some(Background::Color(self.background)),
            border_radius: BORDER_RADIUS,
            border_width: BORDER_WIDTH,
            border_color: Color::BLACK,
        }
    }
}

pub const DEF_TEXT: u16 = 35;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const RECEIPT_WIDTH: u16 = 300;
pub const SQUARE_BUTTON: u16 = 15 + crate::widgets::BIG_TEXT::size();
