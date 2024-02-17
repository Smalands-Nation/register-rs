use iced::{
    widget::{button, container},
    Background, Border, Color,
};

const BORDER: Border = Border {
    radius: 2.0.into(),
    width: 2.0,
    color: Color::BLACK,
};

const NO_BORDER: Border = Border {
    radius: 0.0.into(),
    width: 0.0,
    color: Color::BLACK,
};

pub const DEF_TEXT: f32 = 35.0;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const RECEIPT_WIDTH: f32 = 300.0;
pub const SQUARE_BUTTON: f32 = 15.0 + crate::widgets::BIG_TEXT::size() as f32;

#[derive(Default)]
pub enum Container {
    #[default]
    Empty,
    Border,
    Fill(Color),
    BorderFill(Color),
}

impl From<Container> for iced::theme::Container {
    fn from(value: Container) -> Self {
        Self::Custom(Box::new(value))
    }
}

impl container::StyleSheet for Container {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::BLACK),
            background: match *self {
                Container::Fill(bg) | Container::BorderFill(bg) => Some(Background::Color(bg)),
                _ => None,
            },
            border: match self {
                Container::Border | Container::BorderFill(_) => BORDER,
                _ => NO_BORDER,
            },
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

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::BLACK,
            background: match *self {
                Container::Fill(bg) | Container::BorderFill(bg) => Some(Background::Color(bg)),
                _ => None,
            },
            border: match self {
                Container::Border | Container::BorderFill(_) => BORDER,
                _ => NO_BORDER,
            },
            shadow: Default::default(),
            shadow_offset: Default::default(),
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        Self::active(self, style)
    }
}
