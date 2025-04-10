use giftwrap::Wrap;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::path::Path;
use std::sync::{Arc, LazyLock, OnceLock};
use tokio::sync::Mutex;

macro_rules! sql {
    ($sql:literal, $map_row:expr) => {
        sql!($sql, ::rusqlite::params![], $map_row, _)
    };

    ($sql:literal, $params:expr, $map_row:expr) => {
        sql!($sql, ::rusqlite::params![], $map_row, _)
    };

    ($sql:literal, $map_row:expr, $collect:ty) => {
        sql!($sql, ::rusqlite::params![], $map_row, $collect)
    };

    ($sql:literal, $params:expr, $map_row:expr, $collect:ty) => {
        $crate::CONNECTION
            .get()
            .ok_or($crate::Error::NotConnected)?
            .lock()
            .await
            .prepare_cached($sql)?
            .query_map($params, $map_row)?
            .map(|r| r.map_err($crate::Error::from))
            .collect::<::std::result::Result<$collect, $crate::Error>>()
    };
}

pub mod items;
pub mod receipts;

static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    Migrations::new(vec![
        M::up(include_str!("../db.sql")),
        M::up("DROP TABLE password"),
        M::up(
            r#"
                ALTER TABLE receipt_item ADD COLUMN price INTEGER DEFAULT 1 NOT NULL;
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

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Wrap, Debug, Clone)]
pub enum Error {
    #[giftwrap(wrapDepth = 0)]
    Sqlite(Arc<rusqlite::Error>),
    #[giftwrap(wrapDepth = 0)]
    SqliteMigration(Arc<rusqlite_migration::Error>),
    #[giftwrap(noWrap = true)]
    AlreadyConnected,
    #[giftwrap(noWrap = true)]
    NotConnected,
}
