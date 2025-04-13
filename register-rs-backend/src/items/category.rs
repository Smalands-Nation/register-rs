use {
    rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, ValueRef},
    strum::{Display, VariantArray},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, VariantArray, Display)]
pub enum Category {
    #[strum(to_string = "Alkohol")]
    Alcohol,
    #[strum(to_string = "Dryck")]
    Drink,
    #[strum(to_string = "Mat")]
    Food,
    #[default]
    #[strum(to_string = "Övrigt")]
    Other,
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
