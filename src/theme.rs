use iced::{
    widget::{button, container},
    Background, Color,
};

pub const DEF_TEXT: f32 = 35.0;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const RECEIPT_WIDTH: f32 = 300.0;
pub const ITEM_WIDTH: f32 = 225.0;
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
