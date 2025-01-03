use anyhow::{Ok, Result};
use rusqlite::params;
use std::{fs, path::PathBuf};
use store::Store;
use time::ext::InstantExt;
use time::OffsetDateTime;
pub mod store;
use crate::db::getconn;
use chrono::{DateTime, Local};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Journal {
    pub uuid: Vec<u8>,
    pub uuid_str: String,
    pub buffer_title: String,
    pub path: PathBuf,
    pub buffer: String,
    /// buffer because its always changing
    pub metadata: Metadata,
    pub analysis: Analysis,
}

/// this is sqlite territory. we have some precious metadata
/// journal titles are in the store db for faster gets.
/// all volatile "fields" are functions so we can get it realtime (last_edited, words, etc).
/// do not depend on the struct fields for rt data lol they're just
/// here to pass data unitarily.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub created_at: String,
    pub words: u64,
    pub edited_at: String,
}

pub enum MetadataField {
    edited(String),
    words(u64),
}
impl Metadata {
    // access
    pub fn create(&self, id: Vec<u8>, words: u64) {
        // todo
        let con = getconn();
        // created_at is used for edited_at since its newly created
        match con.execute(
            "INSERT INTO metadata VALUES(?,?,?,?);",
            params![id, self.created_at, self.created_at, words],
        ) {
            core::result::Result::Ok(o) => println!("[METADATA-CREATE] Affected {} rows", o),
            Err(e) => eprintln!("[METADATA-CREATE] {}", e),
        }
    }

    pub fn update(id: Vec<u8>, field: MetadataField) {
        match field {
            MetadataField::edited(datetime) => {
                match getconn().execute(
                    "UPDATE store SET edited_at = ? where uuid = ? ",
                    params![datetime, id],
                ) {
                    core::result::Result::Ok(n) => {
                        println!("[METADATA-UPDATE](edited_at) Affected {} rows", n)
                    }
                    Err(e) => eprintln!("[METADATA-UPDATE] {e}"),
                }
            }
            MetadataField::words(w) => {
                match getconn()
                    .execute("UPDATE store SET words = ? where uuid = ? ", params![w, id])
                {
                    core::result::Result::Ok(n) => {
                        println!("[METADATA-UPDATE](words) Affected {} rows", n)
                    }
                    Err(e) => eprintln!("[METADATA-UPDATE] {e}"),
                }
            }
        }
    }

    // getters
    pub fn last_edited(id: Vec<u8>) -> OffsetDateTime {
        let con = getconn();
        con.query_row(
            "SELECT edited FROM metadata where uuid = ?",
            params![id],
            |row| row.get(0),
        )
        .expect("query failed at Metadata::last_edited()")
    }
    pub fn words(id: Vec<u8>) -> u64 {
        let con = getconn();
        con.query_row(
            "SELECT words FROM metadata WHERE uuid = ? ",
            params![id],
            |row| row.get(0),
        )
        .expect("query failed at Metadata::words()")
    }
}

// some great analysis is on way.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Analysis {}

impl Analysis {
    pub fn new() -> Analysis {
        Analysis {}
    }
}

impl Journal {
    // initalize an existing journal.
    // bad things might happen if u init two same journal at the same time.
    pub fn init(id: Vec<u8>) -> Result<Journal> {
        store::Store::get_journal(id)
    }

    // creates a new journal with the given buffer title
    pub fn new(buffer_title: String) -> Result<Journal> {
        // Journal struct
        let mut st = Store::new()?;

        let id = store::Store::uuid();
        let id_str = id.to_string();

        let date = Local::now().format("%A, %d %B, %Y").to_string();
        let time = Local::now().format(" %-I:%M %p").to_string();
        let created = date + &time;
        let path = store::store_path()?.join(id_str.as_str());

        // Metadata struct
        let meta = Metadata {
            created_at: created.to_string(),
            words: 0,
            edited_at: created.to_string(),
        };

        st.dir.create(id_str.as_str())?;
        // store should always be the first to receive stuff
        st.db.add(
            path.to_string_lossy().to_string(),
            id.as_bytes().to_vec(),
            buffer_title.clone(),
            created,
        );

        // then metadata
        // insert metadata into db
        meta.create(id.as_bytes().to_vec(), 0);
        let journal = Journal {
            uuid: id.as_bytes().to_vec(),
            uuid_str: id.to_string(),
            buffer_title: buffer_title,
            path: path,
            buffer: String::new(),
            metadata: meta,
            analysis: Analysis::new(),
        };
        println!("[NEW-JOURNAL] {:#?}", journal);
        Ok(journal)
    }

    pub fn delete(id: Vec<u8>) -> Result<()> {
        let st = Store::new()?;
        st.db.remove(id);
        Ok(())
    }

    pub fn update_buffer(&mut self, content: String) {
        //! replaces the entire buffer with the new content
        self.buffer = content
    }
    pub fn update_buffer_title(&mut self, title: String) {
        //! replaces the entire buffer with the new content
        self.buffer_title = title
    }

    pub fn write_to_disk(&mut self) -> Result<()> {
        let t = std::time::Instant::now();
        let st = Store::new()?;
        let _ =
            fs::write(self.path.clone(), self.buffer.clone()).map_err(|e| anyhow::Error::from(e));
        st.db
            .update_title(self.buffer_title.clone(), self.uuid.clone());
        println!("[WRITE-TO-DISK] took {}s", t.elapsed().as_secs_f64());
        Ok(())
    }
}

pub struct Media {}
pub enum MediaType {
    Wallpaper(String),
    Emoji(String),
    Font(String),
    Song(String),
}

impl Media {
    /// give a empty mediatype and get that media, from an id
    /// ```
    /// let wp = match Media::get(id.as_bytes().to_vec(), MediaType::Wallpaper(String::new())) {
    /// MediaType::Wallpaper(w) => {w},
    /// _ => {String::new()}
    /// };
    /// ```
    pub fn get(id: Vec<u8>, t: MediaType) -> MediaType {
        let con = getconn();

        match t {
            MediaType::Wallpaper(_) => {
                let r = con
                    .query_row(
                        "SELECT wallpaper FROM media WHERE uuid = ?",
                        params![id],
                        |row| row.get::<usize, String>(0),
                    )
                    .unwrap();
                MediaType::Wallpaper(r)
            }
            MediaType::Emoji(_) => {
                let r = con
                    .query_row(
                        "SELECT emoji FROM media WHERE uuid = ?",
                        params![id],
                        |row| row.get::<usize, String>(0),
                    )
                    .unwrap();
                MediaType::Emoji(r)
            }
            MediaType::Font(_) => {
                let r = con
                    .query_row(
                        "SELECT font FROM media WHERE uuid = ?",
                        params![id],
                        |row| row.get::<usize, String>(0),
                    )
                    .unwrap();
                MediaType::Font(r)
            }
            MediaType::Song(_) => {
                let r = con
                    .query_row(
                        "SELECT song FROM media WHERE uuid = ?",
                        params![id],
                        |row| row.get::<usize, String>(0),
                    )
                    .unwrap();
                MediaType::Song(r)
            }
        }
    }
}
