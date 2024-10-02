use std::process::exit;

use crate::config::utils::db_path;
use chrono;
use rusqlite::{params, Connection, Result};
use time::{Instant, OffsetDateTime, PrimitiveDateTime};

pub fn getconn() -> Connection {
    if let Ok(conn) = Connection::open(db_path().expect("error fetching db path")) {
        let t = chrono::Local::now().format("%-H:%-M:%-S %p").to_string();
        // todo
        // SQLCipher pls
        // encrypting the journals
        println!("\x1b[93m{t}\x1b[0m [GETCONN] opened SQLite connection.");
        conn
    } else {
        if let Err(e) = Connection::open(db_path().expect("error fetching db path")) {
            eprintln!("[SQLITError] {e}");
            exit(1)
        } else {
            exit(0) // this is like opposite of idiomatic its idiotic im sorry
        }
    }
}

pub fn create_tables() {
    let conn = getconn();
    match conn.execute(
        r#"
    CREATE TABLE IF NOT EXISTS store (
        uuid BLOB PRIMARY KEY NOT NULL,
        path TEXT,
        title TEXT,
        os_modified DATETIME
    );
    "#,
        [],
    ) {
        Ok(n) => println!("[SQLITE] Table created.{n} number of rows affected"),
        Err(e) => eprintln!("[SQLITError] {e}"),
    }

    match conn.execute(
        r#"
    CREATE TABLE IF NOT EXISTS media (
        uuid BLOB NOT NULL PRIMARY KEY,
        wallpaper TEXT,
        font TEXT,
        emoji TEXT,
        song TEXT,
        FOREIGN KEY (uuid) REFERENCES store(uuid) ON DELETE CASCADE
    );
    "#,
        [],
    ) {
        Ok(n) => println!("[SQLITE] Table created.{n} number of rows affected"),
        Err(e) => eprintln!("[SQLITError] {e}"),
    }

    match conn.execute(
        r#"
    CREATE TABLE IF NOT EXISTS metadata (
        uuid BLOB PRIMARY KEY NOT NULL,
        created DATETIME NOT NULL,
        edited DATETIME NOT NULL,
        words INTEGER,
        FOREIGN KEY (uuid) REFERENCES store(uuid) ON DELETE CASCADE
    );
    "#,
        [],
    ) {
        Ok(n) => println!("[SQLITE] Table created.{n} number of rows affected"),
        Err(e) => eprintln!("[SQLITError] {e}"),
    }
}
