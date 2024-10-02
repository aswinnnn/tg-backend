use crate::{
    db::{create_tables, getconn},
    journal::store::{self, store_path},
};
use anyhow::{Ok, Result};
use dirs;
use json::Config;
use rusqlite::params;
use std::{collections::HashMap, fs};
use utils::data_path;
pub mod json;
pub mod utils;

pub struct Configuration {}

impl Configuration {
    /// CREATES THE CONFIG DIRECTORY.
    /// see `Configuration::init()` for accessing
    /// the config values.
    pub fn create() -> Result<()> {
        match utils::create_config_dir() {
            Err(e) => {
                eprintln!("\x1b[31m[create-config]\x1b[0m {e}")
            }
            _ => {}
        };

        match utils::populate_config_dir() {
            Err(e) => {
                eprintln!("\x1b[31m[populate-config]\x1b[0m {e}")
            }
            _ => {}
        };

        create_tables();

        Ok(())
    }

    pub fn exists() -> bool {
        match store_path().unwrap().try_exists() {
            core::result::Result::Ok(b) => {
                println!("\x1b[34m[config check]\x1b[32m{b}\x1b[0m");
                b
            }
            Err(e) => {
                eprintln!("\x1b[93m[Error]\x1b[0m {e}");
                return false;
            }
        }
    }

    /// alright, so our settings is stored in a JSON file
    /// in a key-value pair. No we cant use toml because
    /// i want native perfomance when the browser reads the
    /// settings exposed via API (which are modified through this function).
    /// There is also a config table in db updated simultaneously
    /// for faster access to the backend.
    pub fn edit(config: &mut Config, key: String, value: String) {
        match getconn().execute(
            "
        REPLACE INTO config (key,value)
        VALUES(?,?) 
        ",
            params![key, value],
        ) {
            core::result::Result::Ok(o) => {
                println!("\x1b[32m[CONFIG-EDIT-db]\x1b[0m Affected {o} rows.")
            }
            Err(e) => eprintln!("\x1b[31m[CONFIG-EDIT-db]\x1b[0m {e}"),
        };

        json::modify_config(&key, &value, config)
    }
}
