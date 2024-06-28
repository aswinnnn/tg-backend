use std::{collections::HashMap, fs};
use dirs;
use anyhow::{Ok, Result};
use utils::data_path;

use crate::db::create_tables;
pub mod utils;

pub struct Configuration {
    pub configs: HashMap<String, Config>
}

pub enum Config {
    // n
    Number(i64),
    Optioned(Vec<Config>),
    TextValue(String)
}

impl Configuration {
    pub fn new() -> Result<()> {
        match utils::create_config_dir() {
            Err(e) => {eprintln!("[create-config] {e}")},
            _ => {}
        };

        match utils::populate_config_dir() {
            Err(e) => {eprintln!("[populate-config] {e}")},
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
            if e.expect("direntry failed").file_name().to_str().expect("invalid unicode filename wtf") == "config.toml" {
                return true;
            }
            else {continue;}           
        }
        false
    }
}


