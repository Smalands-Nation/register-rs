use giftwrap::Wrap;
use std::sync::Arc;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Wrap, Debug, Clone)]
pub enum Error {
    #[giftwrap(wrapDepth = 0)]
    Sqlite(Arc<rusqlite::Error>),
    #[giftwrap(wrapDepth = 0)]
    SqliteMigration(Arc<rusqlite_migration::Error>),
    IO(std::io::ErrorKind),
    #[giftwrap(wrapDepth = 0)]
    SelfUpdate(Arc<self_update::errors::Error>),
    #[giftwrap(wrapDepth = 0)]
    Tokio(Arc<tokio::task::JoinError>),
    #[giftwrap(wrapDepth = 0)]
    FontError(Arc<iced::font::Error>),
    Other(String),
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        Self::Other(s.into())
    }
}
