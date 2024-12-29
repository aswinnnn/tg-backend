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
            },
        Result::Ok(o) => {println!("\x1b[32m[create-config] success:\x1b[0m {:#?}", o)}
        };

        match utils::populate_config_dir() {
            Err(e) => {
                eprintln!("\x1b[31m[populate-config]\x1b[0m {e}")
            }
            Result::Ok(_) => {
                println!("\x1b[32m[populate-config] success\x1b[0m")
            }
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
    /// in a key-value pair.
    /// 1. Get a Config from read_config()
    /// 2. Put  it through here as one of the arguments to edit it.
    /// 3. This function will take care of writing to the file.
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

        json::modify_config(&key, &value, config);
        if let Err(e) = json::write_config(config) {
            eprintln!("\x1b[31m[Configuration::edit][WRITE-CONFIG]\x1b[0m {e}")
        }
        else {
            println!("[Configuration::edit][WRITE-CONFIG] success")
        }
    }
}
