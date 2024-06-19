use dirs;
use rusqlite::params;
use time::PrimitiveDateTime;
use std::{fs, path::PathBuf};
use anyhow::{Result, Ok,Error};
use crate::{config::utils::data_path, db::getconn};

/// a better rep of the tg directory, where journals are stored.
pub struct Store {
    pub path: PathBuf,
    pub index: Vec<String>,
    pub db: StoreDatabase
}

impl Store {
    pub fn new() -> Result<Store> {
        Ok(Store { path: store()?, index: store_index()?, db: StoreDatabase::new()  })
    }
}


fn store() -> Result<PathBuf> {
    Ok(data_path()?.join("tg"))
}

fn store_index() -> Result<Vec<String>> {
    let mut ve: Vec<String> = Vec::new();
    for entry in store()?.read_dir().expect("failed to read directory") {
        ve.push(entry?.file_name().into_string().expect("invalid unicode filenames have been found! abort!") );
    }
    Ok(ve)
}

pub struct StoreDatabase {
    // this is just to make accessing the store db easy, no fields needed for now
}

impl StoreDatabase {
    pub fn new() -> StoreDatabase {StoreDatabase{}}

    pub fn add(path: String, id: String, datetime: PrimitiveDateTime) {
        let con = getconn();

        match con.execute("INSERT INTO store VALUES(?,?,?);", params![path,id,datetime]) {
            core::result::Result::Ok(n) => println!("[SQLITE](store-add) {n} rows affected"),
            Err(e) => eprintln!("[SQLITError](store-add) {e}"),
        }        
    }

    pub fn remove(id: String) {
        let con = getconn();

        match con.execute("DELETE FROM store WHERE uuid=?;", params![id]) {
            core::result::Result::Ok(n) => println!("[SQLITE][store-remove] {n} rows affected"),
            Err(e) => eprintln!("[SQLITError](store-remove) {e}"),
        }        
    }


}
