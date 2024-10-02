use anyhow::{Ok, Result};
use dirs;
use std::{fs, io::Write, path::PathBuf};

/// `thought-garden-app`
pub fn data_path() -> Result<PathBuf> {
    if let Some(path) = dirs::data_dir() {
        let path = path.join("thought-garden-app");
        Ok(path)
    } else {
        Err(anyhow::format_err!(
            r#"finding the config directory failed. its usually your data roaming folder, example:
        1. [Linux]    /home/alice/.local/share 
        2. [MacOS]    /Users/Alice/Library/Application Support
        3. [Windows]  C:\Users\Alice\AppData\Roaming
        
        this is very unusual. if you can't solve it on your own, open an issue at the github repo."#
        ))
    }
}

/// exact same as store_path, redundant
pub fn journal_path() -> Result<PathBuf> {
    Ok(data_path()?.join("tg"))
}

/// `.db/tgdb`
pub fn db_path() -> Result<PathBuf> {
    Ok(data_path()?.join(".db").join("tgdb"))
}

pub fn create_config_dir() -> Result<PathBuf> {
    let path = data_path()?;
    fs::create_dir(path.clone())?;
    Ok(path)
}

pub fn populate_config_dir() -> Result<()> {
    match fs::File::create(data_path()?.join("config.json")) {
        Result::Ok(mut f) => f.write_all(
            br#"
        {
            "home.emoji": "home",
            "create.emoji": "sunflower"
        }
        "#,
        )?,
        Err(e) => {
            eprintln!("[populat-config-json] {e}")
        }
    }
    fs::create_dir(data_path()?.join("tg"))?;
    fs::create_dir(data_path()?.join(".db"))?;
    fs::File::create(db_path()?)?;
    Ok(())
}
