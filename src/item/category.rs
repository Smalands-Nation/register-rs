use {
    crate::theme::Container,
    iced::Color,
    rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, ValueRef},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Alcohol,
    Drink,
    Food,
    Other,
}

impl Category {
    pub const ALL: [Self; 4] = [Self::Alcohol, Self::Drink, Self::Food, Self::Other];
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alcohol => "Alkohol",
                Self::Drink => "Dryck",
                Self::Food => "Mat",
                Self::Other => "Övrigt",
            }
        )
    }
}

impl FromSql for Category {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(b"alcohol") => Ok(Self::Alcohol),
            ValueRef::Text(b"drink") => Ok(Self::Drink),
            ValueRef::Text(b"food") => Ok(Self::Food),
            ValueRef::Text(b"other") => Ok(Self::Other),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Category {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(match self {
            Self::Alcohol => b"alcohol",
            Self::Drink => b"drink",
            Self::Food => b"food",
            Self::Other => b"other",
        })))
    }
}

impl From<Category> for Container {
    fn from(c: Category) -> Self {
        Self::BorderFill(match c {
            Category::Alcohol => Color::from_rgb8(0xFF, 0x6F, 0x59),
            Category::Drink => Color::from_rgb8(0xC0, 0xDA, 0x74),
            Category::Food => Color::from_rgb8(0xA7, 0xC6, 0xDA),
            Category::Other => Color::WHITE,
        })
    }
}
