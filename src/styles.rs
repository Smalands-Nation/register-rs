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

#[derive(Default, Clone, Copy)]
pub struct TABS;

impl container::StyleSheet for TABS {
    type Style = <iced::Theme as container::StyleSheet>::Style;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::BLACK),
            background: None,
            border_radius: BORDER_RADIUS,
            border_width: BORDER_WIDTH,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl tab_bar::StyleSheet for TABS {
    type Style = <iced::Theme as tab_bar::StyleSheet>::Style;

    fn active(&self, _style: Self::Style, is_active: bool) -> tab_bar::Appearance {
        tab_bar::Appearance {
            background: None,
            border_color: Some(Color::TRANSPARENT),
            border_width: BORDER_WIDTH * 2.0,
            tab_label_background: if is_active {
                Background::Color(Color::WHITE)
            } else {
                Background::Color([0.8, 0.8, 0.8].into())
            },
            tab_label_border_color: Color::TRANSPARENT,
            tab_label_border_width: BORDER_WIDTH,
            icon_color: Color::BLACK,
            text_color: Color::BLACK,
        }
    }

    fn hovered(&self, style: Self::Style, is_active: bool) -> tab_bar::Appearance {
        tab_bar::Appearance {
            tab_label_background: if is_active {
                Background::Color(Color::WHITE)
            } else {
                Background::Color([0.9, 0.9, 0.9].into())
            },
            ..self.active(style, is_active)
        }
    }
}

pub const DEF_TEXT: u16 = 35;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const RECEIPT_WIDTH: u16 = 300;
pub const SQUARE_BUTTON: u16 = 15 + crate::widgets::BIG_TEXT::size();
