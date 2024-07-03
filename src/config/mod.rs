use crate::db::{create_tables, getconn};
use anyhow::{Ok, Result};
use dirs;
use rusqlite::params;
use std::{collections::HashMap, fs};
use toml_edit::{DocumentMut, TomlError};
use utils::data_path;
pub mod utils;

/// every `Config` has a string key.
/// this key maps to a `Config` value that can
/// create, edit that said config.
pub struct Configuration {
}

impl Configuration {
    /// CREATES THE CONFIG DIRECTORY.
    /// see `Configuration::init()` for accessing
    /// the config values.
    pub fn create() -> Result<()> {
        match utils::create_config_dir() {
            Err(e) => {
                eprintln!("[create-config] {e}")
            }
            _ => {}
        };

        match utils::populate_config_dir() {
            Err(e) => {
                eprintln!("[populate-config] {e}")
            }
            _ => {}
        };

        create_tables();

        Ok(())
    }

    pub fn exists() -> bool {
        let r = match fs::read_dir(data_path().expect("checkconfig exists() failed")) {
            core::result::Result::Ok(o) => o,
            Err(_) => return false,
        };

        for e in r {
            if e.expect("direntry failed")
                .file_name()
                .to_str()
                .expect("invalid unicode filename wtf")
                == "config.toml"
            {
                return true;
            } else {
                continue;
            }
        }
        false
    }

    pub fn edit(key: String, value: String) {
        match getconn().execute("
        REPLACE INTO config (key,value)
        VALUES(?,?) 
        ", params![key,value]) {
            core::result::Result::Ok(o) => println!("[CONFIG-EDIT] Affected {o} rows."),
            Err(e) => eprintln!("[CONFIG-EDIT] {e}"),
        };
    }
}