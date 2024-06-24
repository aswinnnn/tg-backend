use std::collections::HashMap;
use dirs;
use anyhow::{Ok, Result};
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
        utils::create_config_dir();
        utils::populate_config_dir();
        Ok(())
    }
}


