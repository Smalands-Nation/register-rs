use giftwrap::Wrap;
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, OnceLock};
use tokio::sync::Mutex;

//Used both for conveinence but also to not leak locked connection
macro_rules! select {
    ($sql:literal, $map_row:expr) => {
        select!($sql, ::rusqlite::params![], $map_row, _)
    };

    ($sql:literal, $map_row:expr, ..) => {
        select!($sql, ::rusqlite::params![], $map_row, ..)
    };

    ($sql:literal, $params:expr, $map_row:expr, ..) => {
        $crate::CONNECTION
            .get()
            .ok_or($crate::Error::NotConnected)?
            .lock()
            .await
            .prepare_cached($sql)?
            .query_map($params, $map_row)?
            .map(|r| r.map_err($crate::Error::from))
    };

    ($sql:literal, $params:expr, $map_row:expr) => {
        select!($sql, ::rusqlite::params![], $map_row, _)
    };

    ($sql:literal, $params:expr, $map_row:expr, $collect:ty) => {
        select!($sql, $params, $map_row, ..)
            .collect::<::std::result::Result<$collect, $crate::Error>>()
    };
}

macro_rules! insert {
    ($sql:literal, $params:expr) => {
        $crate::CONNECTION
            .get()
            .ok_or($crate::Error::NotConnected)?
            .lock()
            .await
            .prepare_cached($sql)?
            .execute($params)
    };
}

pub mod items;
pub mod receipts;
pub mod summary;

static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    Migrations::new(vec![
        M::up(include_str!("../db.sql")),
        M::up("DROP TABLE password"),
        M::up(
            r#"ALTER TABLE receipt_item ADD COLUMN price INTEGER DEFAULT 1 NOT NULL;
               UPDATE receipt_item SET price = (
                   SELECT price FROM menu WHERE receipt_item.item = menu.name
               );
               DROP VIEW receipts_view;
               CREATE VIEW IF NOT EXISTS receipts_view AS
                   SELECT receipts.time, receipt_item.item, receipt_item.amount, receipt_item.price, menu.special, receipts.method 
                   FROM receipts
                       INNER JOIN receipt_item ON receipts.time = receipt_item.receipt
                       INNER JOIN menu ON receipt_item.item = menu.name;
            "#,
        ),
        M::up("ALTER TABLE menu ADD COLUMN category TEXT DEFAULT 'other' NOT NULL;"),
        M::up("UPDATE menu SET name = replace(name, '\u{00A0}', ' ');"),
        M::up(
            r#"UPDATE receipt_item AS r 
                    SET amount = r.price/m.price, price=m.price 
                    FROM menu AS m 
                    WHERE m.name = item AND m.special
            "#,
        ),
    ])
});

static CONNECTION: OnceLock<Mutex<Connection>> = OnceLock::new();

pub fn connect<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut conn = Connection::open(path)?;
    MIGRATIONS.to_latest(&mut conn)?;
    CONNECTION
        .set(Mutex::new(conn))
        .map_err(|_| Error::AlreadyConnected)
}

pub fn set_receipt_path(path: PathBuf) -> Result<()> {
    receipts::print::RECEIPT_PATH
        .set(path)
        .map_err(|_| Error::PathAlreadySet)
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Wrap, Debug, Clone)]
pub enum Error {
    #[giftwrap(wrapDepth = 0)]
    Sqlite(Arc<rusqlite::Error>),
    #[giftwrap(wrapDepth = 0)]
    SqliteMigration(Arc<rusqlite_migration::Error>),
    #[giftwrap(wrapDepth = 0)]
    PrintError(Arc<receipts::print::Error>),
    #[giftwrap(wrapDepth = 0)]
    SummaryError(summary::save::Error),
    #[giftwrap(noWrap = true)]
    AlreadyConnected,
    #[giftwrap(noWrap = true)]
    NotConnected,
    #[giftwrap(noWrap = true)]
    PathAlreadySet,
}
