use crate::error::Result;

pub fn init_db() -> Result<()> {
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
            Ok(backend::connect(conf_path)?)
        }
        None => Err("No config dir".into()),
    }
}

pub fn set_receipt_path() -> Result<()> {
    //FIXME dbg path
    let mut conf_path = dirs::config_dir().ok_or("No config path")?;
    conf_path.push("smaland_register");
    conf_path.push("receipts");
    match std::fs::create_dir_all(&conf_path) {
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => {}
            ek => return Err(ek.into()),
        },
        _ => {}
    }
    Ok(backend::set_receipt_path(conf_path)?)
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
pub fn update() -> Result<self_update::Status> {
    Ok(self_update::Status::UpToDate("Dev".into()))
}
