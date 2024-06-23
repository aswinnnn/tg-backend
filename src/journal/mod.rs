use std::{fs, path::PathBuf};
use time::ext::InstantExt;
use anyhow::{Ok,Result};
use rusqlite::params;
use store::Store;
use time::OffsetDateTime;
pub mod store;
use once_cell::sync::{Lazy, OnceCell};
use uuid::Uuid;

use crate::db::getconn;


#[derive(Debug)]
struct Journal {
    uuid: Vec<u8>,
    buffer_title: String,
    path: PathBuf,
    buffer: String, /// buffer because its always changing
    metadata: Metadata,
    analysis: Analysis
}

/// this is sqlite territory. we have some precious metadata
/// journal titles are in the store db for faster gets.
/// all volatile "fields" are functions so we can get it realtime.
#[derive(Debug)]
pub struct Metadata {
    pub created_at: String
}
pub enum MetadataField {
    edited(OffsetDateTime),
    words(u64)
}
impl Metadata {
    // access
    pub fn create(&self, id: Vec<u8>, words: u64) {
        let con = getconn();
        // created_at is used for edited_at since its newly created
        match con.execute("INSERT INTO metadata VALUES(?,?,?);", params![id,self.created_at,words]) {
            core::result::Result::Ok(o) => println!("[METADATA-CREATE] Affected {} rows", o),
            Err(e) => eprintln!("[METADATA-CREATE] {}",e),
        }
    }

    pub fn update(id: Vec<u8>,field: MetadataField) {
        match field {
            MetadataField::edited(datetime) => {
                match getconn().execute("UPDATE store SET edited_at = ? where uuid = ? ", params![datetime, id]) {
                    core::result::Result::Ok(n) => println!("[METADATA-UPDATE](edited_at) Affected {} rows", n),
                    Err(e) => eprintln!("[METADATA-UPDATE] {e}"),
                }
            },
            MetadataField::words(w) => {
                match getconn().execute("UPDATE store SET words = ? where uuid = ? ", params![w, id]) {
                    core::result::Result::Ok(n) => println!("[METADATA-UPDATE](words) Affected {} rows", n),
                    Err(e) => eprintln!("[METADATA-UPDATE] {e}"),
                }
            },
        }
    }

    // getters
    pub fn last_edited(id: Vec<u8>) -> OffsetDateTime {
        let con = getconn();
        con.query_row("SELECT edited FROM metadata where uuid = ?", params![id], |row| {
            row.get(0)
        }).expect("query failed at Metadata::last_edited()") 
    }
    pub fn words(id: Vec<u8>) -> u64{
        let con = getconn();
        con.query_row("SELECT words FROM metadata WHERE uuid = ? ", params![id], 
    |row| {
        row.get(0)
    }).expect("query failed at Metadata::words()")
    }
}

// some great analysis is on way.
#[derive(Debug)]
struct Analysis {}

impl Analysis {
    pub fn new() -> Analysis {
        Analysis {}
    }
}


impl Journal {
    // creates a new journal with the given buffer title

    pub fn new(buffer_title: String) -> Result<Journal> {

        // Journal struct
        let mut st = Store::new()?;

        let id = store::Store::uuid();
        let id_str = id.to_string();
        let created = OffsetDateTime::now_utc();
        let path = store::store_path()?.join(id_str.as_str());

        // Metadata struct
        let meta = Metadata { created_at: created.to_string() };
        meta.create(id.as_bytes().to_vec(), 0);

        st.dir.create(id_str.as_str())?;
        st.db.add(path.to_string_lossy().to_string(), id.as_bytes().to_vec(), buffer_title.clone(), created);
        
        let journal = Journal { 
            uuid: id.as_bytes().to_vec(),
            buffer_title: buffer_title, 
            path: path,
            buffer: String::new(),
            metadata: meta,
            analysis: Analysis::new() };
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

    fn write_to_disk(&mut self) -> Result<()> {
        let t = std::time::Instant::now();
        let st = Store::new()?;
        let _ = fs::write(self.path.clone(), self.buffer.clone()).map_err(|e| {
            anyhow::Error::from(e)  
        });
        st.db.update_title(self.buffer_title.clone(), self.uuid.clone());
        println!("[WRITE-TO-DISK] took {}s", t.elapsed().as_secs_f64());
        Ok(())
    }

}