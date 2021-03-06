use iced::{
    pure::{widget::Text, Element},
    Font,
};

pub const ICON_FONT: Font = Font::External {
    name: "icofont",
    bytes: include_bytes!("../resources/google-fonts-icons.ttf"),
};

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

impl From<Icon> for Text {
    fn from(i: Icon) -> Text {
        Text::new(i).font(ICON_FONT)
    }
}

impl<'a, M> From<Icon> for Element<'a, M> {
    fn from(i: Icon) -> Self {
        Text::from(i).into()
    }
}
