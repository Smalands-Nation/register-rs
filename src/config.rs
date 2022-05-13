use {crate::error::Result, rusqlite::Connection};

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
            let conn = Connection::open(conf_path)?;
            conn.execute_batch(include_str!("../db.sql"))?;
            Ok(conn)
        }
        None => Err("No config dir".into()),
    }
}

#[cfg(not(debug_assertions))]
use self_update::{backends::github, cargo_crate_version};

#[cfg(not(debug_assertions))]
pub fn update() -> Result<()> {
    let status = github::Update::configure()
        .repo_owner("Smalands-Nation")
        .repo_name("register-rs")
        .bin_name("smalands-rs")
        .show_output(false)
        .no_confirm(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}

#[cfg(debug_assertions)]
pub fn update() -> Result<()> {
    Ok(())
}
