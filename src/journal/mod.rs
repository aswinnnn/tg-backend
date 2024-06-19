use std::{fs, path::PathBuf};

use time::PrimitiveDateTime;

pub mod store;

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
struct Metadata {
    created_at: String
}

impl Metadata {
    pub fn title() {}
    pub fn last_edited() {}
    pub fn words() {}
    pub fn wallpaper() {}
}

struct Analysis {
}


impl Journal {
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