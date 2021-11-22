use iced::{container, Background, Color};

pub struct Container {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: self.text_color,
            background: self.background,
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
        }
    }
}

pub const BIG_TEXT: u16 = 45;
pub const DEF_TEXT: u16 = 35;
pub const SMALL_TEXT: u16 = 20;

pub const DEF_PADDING: u16 = 10;
pub const SMALL_PADDING: u16 = 5;

pub const BORDERED: Container = Container {
    text_color: Some(Color::BLACK),
    background: None,
    border_radius: 2f32,
    border_width: 2f32,
    border_color: Color::BLACK,
};

pub const RECEIPT_WIDTH: u16 = 300;
pub const SQUARE_BUTTON: u16 = 15 + BIG_TEXT;
