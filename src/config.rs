use {
    crate::error::Result,
    lazy_static::lazy_static,
    rusqlite::Connection,
    rusqlite_migration::{Migrations, M},
};

lazy_static! {
    static ref MIGRATIONS: Migrations<'static> = Migrations::new(vec![
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
            "#
        ),
    ]);
}

pub fn init_db() -> Result<Connection> {
    match dirs::config_dir() {
        Some(mut conf_path) => {
            conf_path.push("smaland_register");
            match std::fs::create_dir_all(&conf_path) {
                Ok(_) => (),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::AlreadyExists => (),
                    ek => return Err(ek.into()),
                },
            };
            conf_path.push("db.db");
            let mut conn = Connection::open(conf_path)?;

            MIGRATIONS.to_latest(&mut conn)?;

            Ok(conn)
        }
        None => Err("No config dir".into()),
    }
}

#[cfg(not(debug_assertions))]
use self_update::{backends::github, cargo_crate_version, Status};

#[cfg(not(debug_assertions))]
pub fn update() -> Result<Status> {
    let status = github::Update::configure()
        .repo_owner("Smalands-Nation")
        .repo_name("register-rs")
        .bin_name("smalands-rs")
        .show_output(false)
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    Ok(status)
}

#[cfg(debug_assertions)]
pub fn update() -> Result<()> {
    Ok(())
}
