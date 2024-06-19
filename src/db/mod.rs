use std::process::exit;

use rusqlite::{params, Connection, Result};
use time::PrimitiveDateTime;
use crate::config::utils::db_path;

pub fn getconn() -> Connection {
    if let Ok(conn) = Connection::open(db_path().expect("error fetching db path")) {
        // todo
        // figure out how to get the time NOW and log it here
        // StoreFolder impls
        // use both Store* in Journal to make Journal impls
        // figure out uuids
        // SQLCipher pls
        // encrypting the journals
        println!("[CONNECTION-OPEN]");
        conn
    }
    else { 
        if let Err(e) = Connection::open(db_path().expect("error fetching db path")) {
            eprintln!("[SQLITError] {e}");
            exit(1)
        }
        else {
            exit(0) // this is like opposite of idiomatic its idiotic im sorry
        }
    }
}

pub fn create_tables() {
    let conn = getconn();
    match conn.execute(r#"
    CREATE TABLE IF NOT EXISTS metadata (
        uuid NOT NULL PRIMARY KEY,
        created DATETIME NOT NULL,
        edited DATETIME NOT NULL,
        words INTEGER
    );

    CREATE TABLE IF NOT EXISTS media (
        uuid NOT NULL FOREIGN KEY,
        wallpaper TEXT,
        font TEXT,
        emoji TEXT,
        song TEXT
    );
    CREATE TABLE IF NOT EXISTS store (
        path TEXT,
        uuid FOREIGN KEY,
        os_modified DATETIME
    );
    "#, []) {
        Ok(n) => println!("[SQLITE] Tables created.{n} number of rows affected"),
        Err(e) => eprintln!("[SQLITError] {e}")
    }
}