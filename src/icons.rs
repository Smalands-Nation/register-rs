use iced::{Font, Text};

pub const ICON_FONT: Font = Font::External {
    name: "icofont",
    bytes: include_bytes!("../resources/icofont.ttf"),
};

pub enum Icon {
    Trash,
    Menu,
    Settings,
    Cross,
    Reciept,
    Print,
    Left,
    Right,
}

impl From<Icon> for char {
    fn from(i: Icon) -> Self {
        match i {
            Icon::Trash => '\u{eebb}',
            Icon::Menu => '\u{eb8b}',
            Icon::Settings => '\u{efe1}',
            Icon::Cross => '\u{eee4}',
            Icon::Reciept => '\u{ef72}',
            Icon::Print => '\u{efc6}',
            Icon::Left => '\u{ea9d}',
            Icon::Right => '\u{eaa0}',
        }
    }
}

impl From<Icon> for Text {
    fn from(i: Icon) -> Text {
        Text::new(char::from(i).to_string()).font(ICON_FONT)
    }
}
