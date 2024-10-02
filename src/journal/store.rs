use crate::config;
use crate::{
    analysis,
    config::{
        utils::{data_path, populate_config_dir},
        Configuration,
    },
    db::getconn,
    journal::{Journal, Metadata},
};
use anyhow::{Error, Ok, Result};
use axum::body::Bytes;
use dirs;
use rand::seq::SliceRandom;
use rand::Rng;
use rusqlite::params;
use std::{
    fs::{self, File},
    path::PathBuf,
    process::exit,
};
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

/// gives you access to the journal database and directory
pub struct Store {
    pub path: PathBuf,
    pub index: Vec<String>,
    pub db: StoreDatabase,
    pub dir: StoreFolder,
    pub config: Configuration,
}

impl Store {
    pub fn new() -> Result<Store> {
        Ok(Store {
            path: store_path()?,
            index: store_index()?,
            db: StoreDatabase::new(),
            dir: StoreFolder::new(),
            config: Configuration {},
        })
    }

    pub fn get_journal(id: Vec<u8>) -> Result<Journal> {
        let con = getconn();
        let id_str = Uuid::from_slice(&id)
            .expect("error making uuid")
            .to_string();

        // journal struct
        let mut title = String::new();
        let mut path = String::new();
        let mut content =
            fs::read_to_string(store_path()?.join(Uuid::from_slice(&id).unwrap().to_string()))
                .unwrap();

        // metadata struct
        let mut created = String::from(" ");
        let mut edited = String::from(" ");
        let mut words = 0;

        let _ = con.query_row(
            "SELECT path, title FROM store WHERE uuid = ?",
            params![id],
            |row| {
                path = row.get(0).expect("[sqlite] failed to get path");
                title = row.get(1).expect("[sqlite] failed to get title");
                core::result::Result::Ok(())
            },
        );

        let _ = con.query_row(
            "SELECT created,edited,words FROM metadata WHERE uuid = ?",
            params![id],
            |row| {
                created = row.get(0).expect("[sqlite] failed to get created_at");
                edited = row.get(1).expect("[sqlite] failed to get edited");
                words = row.get(2).expect("[sqlite] failed to get words");
                core::result::Result::Ok(())
            },
        );

        let meta = Metadata {
            created_at: created,
            words: words,
            edited_at: edited,
        };

        let j = Journal {
            uuid: id,
            uuid_str: id_str.clone(),
            buffer_title: title,
            path: path.into(),
            buffer: content,
            metadata: meta,
            analysis: super::Analysis {},
        };

        println!("[GOT JOURNAL] {}", id_str);
        Ok(j)
    }

    pub fn uuid() -> Uuid {
        uuid::Uuid::new_v4()
    }
}

/// journal files are stored here.
pub fn store_path() -> Result<PathBuf> {
    Ok(data_path()?.join("tg"))
}

fn store_index() -> Result<Vec<String>> {
    let mut ve: Vec<String> = Vec::new();
    let r = store_path()?
        .read_dir()
        .expect("LOOKS LIKE THE CONFIG DIRECTORY MIGHT BE GONE OR INACCESSIBLE.A");

    for entry in r {
        ve.push(
            entry?
                .file_name()
                .into_string()
                .expect("invalid unicode filenames have been found! abort!"),
        );
    }
    Ok(ve)
}

pub struct StoreDatabase {
    // this is just to make accessing the store db easy, no fields needed for now
}

impl StoreDatabase {
    pub fn new() -> StoreDatabase {
        StoreDatabase {}
    }

    pub fn add(self, path: String, id: Vec<u8>, title: String, datetime: String) {
        let con = getconn();

        match con.execute(
            "INSERT INTO store VALUES(?,?,?,?);",
            params![id, path, title, datetime],
        ) {
            core::result::Result::Ok(n) => println!("[SQLITE](store-add) {n} rows affected"),
            Err(e) => eprintln!("[SQLITError](store-add) {e}"),
        }

        let random_wallpaper = format!("assets/{}.avif", rand::thread_rng().gen_range(1..=15));
        // media should be shallow populated otherwise it just...errs
        match con.execute(
            "INSERT INTO media VALUES(?,?,?,?,?);",
            params![
                id,
                random_wallpaper,
                String::from("Lexend"),
                String::from("🌿"),
                String::from("")
            ],
        ) {
            core::result::Result::Ok(n) => println!("[SQLITE](media-add) {n} rows affected"),
            Err(e) => eprintln!("[SQLITError](store-add) {e}"),
        }
    }

    pub fn remove(self, id: Vec<u8>) {
        let con = getconn();

        match con.execute("DELETE FROM store WHERE uuid=?;", params![id]) {
            core::result::Result::Ok(n) => println!("[SQLITE][store-remove] {n} rows affected"),
            Err(e) => eprintln!("[SQLITError](store-remove) {e}"),
        }
    }

    pub fn update_title(&self, title: String, id: Vec<u8>) {
        let con = getconn();

        match con.execute(
            "UPDATE store SET title = ? WHERE uuid = ?;",
            params![title, id],
        ) {
            core::result::Result::Ok(n) => println!("[SQLITE][store-update] {n} rows affected"),
            Err(e) => eprintln!("[SQLITError](store-update) {e}"),
        }
    }
}

pub struct StoreFolder {}

impl StoreFolder {
    pub fn new() -> StoreFolder {
        StoreFolder {}
    }

    /// creates a file in the tg [journal] folder
    pub fn create(&mut self, id: &str) -> Result<()> {
        let f = File::create(store_path()?.join(id))?;
        Ok(())
    }

    /// deletes a file in the tg [journal] folder
    pub fn delete(&mut self, id: &str) -> Result<()> {
        fs::remove_file(store_path()?.join(id))?;
        Ok(())
    }

    /// OVERWRITES ALL CONTENT with the new content.
    pub fn write(&mut self, id: &str, content: String) -> Result<()> {
        fs::write(store_path()?.join(id), content.clone())?;
        let len: usize = content.chars().count();
        Ok(())
    }
}
