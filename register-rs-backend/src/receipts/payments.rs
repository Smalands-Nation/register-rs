use {
    rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, ValueRef},
    strum::{Display, VariantArray},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Display, VariantArray)]
pub enum Payment {
    Cash,
    #[default]
    Swish,
    Paypal,
}

impl FromSql for Payment {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(b"cash") => Ok(Self::Cash),
            ValueRef::Text(b"swish") => Ok(Self::Swish),
            ValueRef::Text(b"paypal") => Ok(Self::Paypal),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Payment {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(match self {
            Self::Cash => b"cash",
            Self::Swish => b"swish",
            Self::Paypal => b"paypal",
        })))
    }
}
