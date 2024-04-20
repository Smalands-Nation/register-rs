use {
    iced::{
        widget::{button, container},
        Background, Color,
    },
    iced_aw::style::tab_bar,
};

pub const DEF_TEXT: f32 = 33.0;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const RECEIPT_WIDTH: f32 = 300.0;
pub const SQUARE_BUTTON: f32 = 25.0 + crate::widgets::BIG_TEXT::size() as f32;

#[derive(Default, Clone, Copy)]
pub enum Container {
    #[default]
    Empty,
    Border,
    Fill(Color),
    BorderFill(Color),
}

impl From<Container> for iced::Border {
    fn from(value: Container) -> Self {
        match value {
            Container::Border | Container::BorderFill(_) => iced::Border {
                radius: 2.0.into(),
                width: 2.0,
                color: Color::BLACK,
            },
            _ => iced::Border {
                radius: 0.0.into(),
                width: 0.0,
                color: Color::BLACK,
            },
        }
    }
}

impl From<Container> for iced::theme::Container {
    fn from(value: Container) -> Self {
        Self::Custom(Box::new(value))
    }
}

impl container::StyleSheet for Container {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::BLACK),
            background: match *self {
                Container::Fill(bg) | Container::BorderFill(bg) => Some(Background::Color(bg)),
                _ => None,
            },
            border: (*self).into(),
            shadow: Default::default(),
        }
    }
}

impl From<Container> for iced::theme::Button {
    fn from(value: Container) -> Self {
        Self::Custom(Box::new(value))
    }
}

impl button::StyleSheet for Container {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::BLACK,
            background: match *self {
                Container::Fill(bg) | Container::BorderFill(bg) => Some(Background::Color(bg)),
                _ => None,
            },
            border: (*self).into(),
            shadow: Default::default(),
            shadow_offset: Default::default(),
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        Self::active(self, style)
    }
}

pub struct TabStyle;

impl From<TabStyle> for tab_bar::TabBarStyles {
    fn from(value: TabStyle) -> Self {
        Self::Custom(std::rc::Rc::new(value))
    }
}

impl tab_bar::StyleSheet for TabStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style, is_active: bool) -> tab_bar::Appearance {
        tab_bar::Appearance {
            border_color: Some(Color::TRANSPARENT),
            border_width: 4.0,
            tab_label_background: if is_active {
                Background::Color(Color::WHITE)
            } else {
                Background::Color([0.8, 0.8, 0.8].into())
            },
            tab_label_border_color: Color::TRANSPARENT,
            tab_label_border_width: 2.0,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style, is_active: bool) -> tab_bar::Appearance {
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
