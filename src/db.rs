use rusqlite::{Connection, Result, params};
use crate::ctf;
use crate::settings;
use chrono::{DateTime, Utc};

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
            flag TEXT NOT NULL,
            FOREIGN KEY(ctf_id) REFERENCES ctf(id)
        )",
        params![],
    ).unwrap();
}

pub fn get_conn() -> Connection {
    Connection::open(settings::DB_FILE).unwrap()
}

pub fn get_ctf_from_id(id: String) -> Result<ctf::Ctf> {
    let conn = get_conn();
    let mut stmt = conn.prepare("SELECT path, name, url, creds, start, end FROM ctf WHERE id = ?1")?;
    let ctf_iter = stmt.query_map(params![id], |row| {
        Ok(ctf::Ctf::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            (row.get(3)?, row.get(4)?),
            row.get::<usize, String>(5)?.parse().unwrap(),
            row.get::<usize, String>(6)?.parse().unwrap(),
        ))
    })?;

    for ctf in ctf_iter {
        return ctf;
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_challenge_from_id(id: String) -> Result<ctf::challenge::Challenge> {
    let conn = get_conn();
    let mut stmt = conn.prepare("SELECT ctf_id, name, flag FROM challenge WHERE id = ?1")?;
    let challenge_iter = stmt.query_map(params![id], |row| {
        Ok(ctf::challenge::Challenge::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    })?;

    for challenge in challenge_iter {
        return challenge;
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}