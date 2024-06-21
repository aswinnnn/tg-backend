use std::{fs, path::PathBuf};

use anyhow::{Ok,Result};
use time::OffsetDateTime;
pub mod store;

static STORE: store::Store = store::Store::new().expect("failed to get journal storage.");

#[derive(Debug)]
struct Journal {
    buffer_title: String,
    path: PathBuf,
    buffer: String, /// buffer because its always changing
    metadata: Metadata,
    analysis: Analysis
}

/// this is sqlite territory. we have some precious metadata
/// which includes the journal title
/// all volatile "fields" are functions so we can get it realtime.
#[derive(Debug)]
struct Metadata {
    created_at: String
}

impl Metadata {
    pub fn last_edited() {}
    pub fn words() {}
    pub fn wallpaper() {}
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
        let mut st = store::Store::new()?;
        let id = store::Store::uuid();
        let id_str = id.to_string();
        let created = OffsetDateTime::now_utc();
        let path = store::store_path()?.join(id_str.as_str());

        // Metadata struct
        let meta = Metadata { created_at: created.to_string() };

        st.dir.create(id_str.as_str());
        st.db.add(path.to_string_lossy().to_string(), id.as_bytes().to_vec(), created);
        
        let journal = Journal { buffer_title: buffer_title, 
            path: path,
            buffer: String::new(),
            metadata: meta,
            analysis: Analysis::new() };
        println!("[NEW-JOURNAL] {:#?}", journal);
        Ok(journal)  
        
    }

    pub fn delete() -> Result<()> {
        Ok(())
    }

    pub fn update_buffer(mut self, content: String) {
        //! replaces the entire buffer with the new content
        self.buffer = content
    }
    pub fn update_buffer_title(mut self, title: String) {
        //! replaces the entire buffer with the new content
        self.buffer_title = title
    }

    fn write(self) -> Result<(), std::io::Error> {
        fs::write(self.path, self.buffer)
    }

}