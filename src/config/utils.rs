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
        
        this is very unusual. if you can't solve it on your own, open an issue in github. (https://github.com/aswinnnn/thought-garden)."#
        ))
    }
}

/// exact same as store_path, redundant
pub fn journal_path() -> Result<PathBuf> {
    Ok(data_path()?.join("tg"))
}

/// `.db/tgdb` our database file
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
            br####"
        {
            "home.emoji": "&#x1f3e1",
            "create.emoji": "&#x1f33b",
            "theme.light.background-color":"#ffcfa8",
            "theme.light.text-color": "#00000",
            "theme.dark.background-color": "#3d3128",
            "theme.dark.text-color": "#ffffff"
        }
        "####,
        )?,
        Err(e) => {
            eprintln!("[populate-config-json] {e}")
        }
    }
    fs::create_dir(data_path()?.join("tg"))?;
    fs::create_dir(data_path()?.join(".db"))?;
    fs::File::create(db_path()?)?;
    Ok(())
}

/// pass only one emoji without any spaces ok
fn emoji_to_ascii(s: &str) -> String {

    let mut r = String::new();
    let _ = s.chars().map(|c| {
        if c.is_ascii() {r = c.to_string();}
        else {r = format!("0x{:X}",c as u32);}
    });
    r
}