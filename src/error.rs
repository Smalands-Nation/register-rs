use giftwrap::Wrap;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Wrap, Debug, Clone)]
pub enum Error {
    #[wrapDepth(0)]
    Sqlite(Arc<rusqlite::Error>),
    IO(std::io::ErrorKind),
    #[wrapDepth(0)]
    SelfUpdate(Arc<self_update::errors::Error>),
    Other(String),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Self::Other(s.into())
    }
}
