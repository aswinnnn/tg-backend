use serde_json::{from_reader, to_writer, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use super::utils::data_path;

pub type Config = HashMap<String, Value>;

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open(data_path()?.join("config.json"))?;
    let reader = BufReader::new(file);
    let config: Config = from_reader(reader)?;
    Ok(config)
}

pub fn modify_config(key: &str, value: &str, config: &mut Config) {
    // Add a new key-value pair or update existing one
    match config.insert(key.to_string(), Value::String(value.to_string())) {
        Some(old) => {
            println!("\x1b[97m[MODIFY-CONFIG]\x1b[0m updated key '{key}' from '{old}' to '{value}'")
        }
        None => {
            println!("\x1b[96m[MODIFY-CONFIG]\x1b[0m new key: {key} value: {value}");
        }
    };
}

pub fn write_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("config.json")?;
    let writer = BufWriter::new(file);
    to_writer(writer, config)?;
    Ok(())
}
