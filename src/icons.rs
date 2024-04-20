use {
    crate::widgets::BIG_TEXT,
    iced::{
        alignment::{Horizontal, Vertical},
        widget::Text,
        Element, Font,
    },
};

pub const ICON_FONT: Font = Font::with_name("Material Symbols Outlined 48pt");

pub enum Icon {
    Trash,
    Menu,
    Settings,
    Cross,
    Receipt,
    Print,
    Left,
    Right,
    Money,
    Lock,
    Info,
}

impl From<Icon> for char {
    fn from(i: Icon) -> Self {
        match i {
            Icon::Trash => '\u{e872}',
            Icon::Menu => '\u{e561}',
            Icon::Settings => '\u{e8b8}',
            Icon::Cross => '\u{e5cd}',
            Icon::Receipt => '\u{ef6e}',
            Icon::Print => '\u{e8ad}',
            Icon::Left => '\u{e408}',
            Icon::Right => '\u{e409}',
            Icon::Money => '\u{ef63}',
            Icon::Lock => '\u{e897}',
            Icon::Info => '\u{e88e}',
        }
    }
}

impl From<Icon> for String {
    fn from(i: Icon) -> Self {
        char::from(i).to_string()
    }
}

impl<'a> From<Icon> for Text<'a> {
    fn from(i: Icon) -> Text<'a> {
        BIG_TEXT::new(String::from(i))
            .font(ICON_FONT)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
    }
}

impl<'a, M> From<Icon> for Element<'a, M>
where
    M: 'a,
{
    fn from(i: Icon) -> Self {
        Text::from(i).into()
    }
}
