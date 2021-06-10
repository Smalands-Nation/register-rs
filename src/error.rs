use giftwrap::Wrap;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Wrap, Debug)]
pub enum Error {
    Sqlite(rusqlite::Error),
    Other(String),
}
