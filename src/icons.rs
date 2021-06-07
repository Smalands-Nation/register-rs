use iced::{Font, Text};

pub const ICON_FONT: Font = Font::External {
    name: "icofont",
    bytes: include_bytes!("../resources/icofont.ttf"),
};

pub enum Icon {
    Trash,
    Menu,
    Settings,
}

impl From<Icon> for char {
    fn from(i: Icon) -> Self {
        match i {
            Icon::Trash => '\u{eebb}',
            Icon::Menu => '\u{eb8b}',
            Icon::Settings => '\u{efe1}',
        }
    }
}

impl From<Icon> for Text {
    fn from(i: Icon) -> Text {
        Text::new(char::from(i).to_string()).font(ICON_FONT)
    }
}
