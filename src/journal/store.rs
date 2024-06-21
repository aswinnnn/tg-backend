use axum::body::Bytes;
use dirs;
use rusqlite::params;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;
use std::{fs::{self, File}, path::PathBuf};
use anyhow::{Result, Ok,Error};
use crate::{config::utils::data_path, db::getconn};

/// a better rep of the tg directory, where journals are stored.
/// gives you access to the journal database and directory
pub struct Store {
    pub path: PathBuf,
    pub index: Vec<String>,
    pub db: StoreDatabase,
    pub dir: StoreFolder
}

impl Store {
    pub fn new() -> Result<Store> {
        Ok(Store { path: store_path()?, index: store_index()?, db: StoreDatabase::new(), dir: StoreFolder::new()  })
    }

    pub fn uuid() -> Uuid {
        uuid::Uuid::new_v4()
    }
}


pub fn store_path() -> Result<PathBuf> {
    Ok(data_path()?.join("tg"))
}

fn store_index() -> Result<Vec<String>> {
    let mut ve: Vec<String> = Vec::new();
    for entry in store_path()?.read_dir().expect("failed to read directory") {
        ve.push(entry?.file_name().into_string().expect("invalid unicode filenames have been found! abort!") );
    }
    Ok(ve)
}

struct StoreDatabase {
    // this is just to make accessing the store db easy, no fields needed for now
}

impl StoreDatabase {
    pub fn new() -> StoreDatabase {StoreDatabase{}}

    pub fn add(self, path: String, id: Vec<u8>, datetime: OffsetDateTime) {
        let con = getconn();

        match con.execute("INSERT INTO store VALUES(?,?,?);", params![path,id,datetime]) {
            core::result::Result::Ok(n) => println!("[SQLITE](store-add) {n} rows affected"),
            Err(e) => eprintln!("[SQLITError](store-add) {e}"),
        }        
    }

    pub fn remove(self, id: String) {
        let con = getconn();

        match con.execute("DELETE FROM store WHERE uuid=?;", params![id]) {
            core::result::Result::Ok(n) => println!("[SQLITE][store-remove] {n} rows affected"),
            Err(e) => eprintln!("[SQLITError](store-remove) {e}"),
        }        
    }

}

pub struct StoreFolder {
    // access system journal directory using methods.
    // fields are statistics [no. of journals] related to the current session.
    pub deleted: u64,
    pub created: u64,
    pub written_words: u64
}

impl StoreFolder {
    pub fn new() -> StoreFolder {
        StoreFolder { deleted: 0, created: 0, written_words: 0 }
    }

    /// creates a file in the tg [journal] folder
    pub fn create(&mut self, id: &str) -> Result<()> {
        let f = File::create(store_path()?.join(id))?;
        self.created += 1;
        Ok(())
    }

    /// deletes a file in the tg [journal] folder
    pub fn delete(&mut self,id: &str) -> Result<()> {
        fs::remove_file(store_path()?.join(id))?;
        self.deleted += 1;
        Ok(())
    }

    /// OVERWRITES ALL CONTENT with the new content.
    pub fn write(&mut self, id: &str, content: String) -> Result<()> {

        fs::write(store_path()?.join(id), content.clone())?;
        let len: usize = content.chars().count();
        self.written_words += len as u64;
        Ok(())
    }
}
