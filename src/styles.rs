use {
    iced::{container, Background, Color},
    iced_aw::style::tab_bar,
};

const BORDER_RADIUS: f32 = 2.0;
pub const BORDER_WIDTH: f32 = 2.0;

pub struct BORDERED;

impl container::StyleSheet for BORDERED {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::BLACK),
            background: None,
            border_radius: BORDER_RADIUS,
            border_width: BORDER_WIDTH,
            border_color: Color::BLACK,
        }
    }
}

pub struct TABS;

impl container::StyleSheet for TABS {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::BLACK),
            background: None,
            border_radius: BORDER_RADIUS,
            border_width: BORDER_WIDTH,
            border_color: Color::TRANSPARENT,
        }
    }
}

impl tab_bar::StyleSheet for TABS {
    fn active(&self, is_active: bool) -> tab_bar::Style {
        tab_bar::Style {
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

    fn hovered(&self, is_active: bool) -> tab_bar::Style {
        tab_bar::Style {
            tab_label_background: if is_active {
                Background::Color(Color::WHITE)
            } else {
                Background::Color([0.9, 0.9, 0.9].into())
            },
            ..self.active(is_active)
        }
    }
}

pub const BIG_TEXT: u16 = 45;
pub const DEF_TEXT: u16 = 35;
pub const SMALL_TEXT: u16 = 20;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const RECEIPT_WIDTH: u16 = 300;
pub const SQUARE_BUTTON: u16 = 15 + BIG_TEXT;
