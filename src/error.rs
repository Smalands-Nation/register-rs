use giftwrap::Wrap;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Wrap, Debug, Clone)]
pub enum Error {
    #[noWrap]
    Sqlite(Arc<rusqlite::Error>),
    IO(std::io::ErrorKind),
    Other(String),
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Self::Sqlite(Arc::new(e))
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Self::Other(s.into())
    }
}
