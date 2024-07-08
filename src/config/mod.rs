use crate::{db::{create_tables, getconn}, journal::store::{self, store_path}};
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
        match store_path().unwrap().try_exists() {
            core::result::Result::Ok(b) => {
                println!("\x1b[34m[config check]\x1b[32m{b}\x1b[0m"); b
            },
            Err(e) => {eprintln!("\x1b[93m[Error]\x1b[0m {e}"); return false},
        }

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