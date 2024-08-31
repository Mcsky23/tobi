use rusqlite::{Connection, Result, params};
use crate::ctf;
use crate::settings;

pub fn init() {
    // Create a new SQLite database
    let conn = Connection::open(settings::DB_FILE).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ctf (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            creds TEXT NOT NULL,
            start TEXT NOT NULL,
            end TEXT NOT NULL
        )",
        params![],
    ).unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS challenge (
            id INTEGER PRIMARY KEY,
            ctf_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            points INTEGER NOT NULL,
            flag TEXT NOT NULL,
            FOREIGN KEY(ctf_id) REFERENCES ctf(id)
        )",
        params![],
    ).unwrap();
}

pub fn get_conn() -> Connection {
    Connection::open(settings::DB_FILE).unwrap()
}