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
            category TEXT NOT NULL,
            flag TEXT NOT NULL,
            FOREIGN KEY(ctf_id) REFERENCES ctf(id)
        )",
        params![],
    ).unwrap();
}

pub fn get_conn() -> Connection {
    Connection::open(settings::DB_FILE).unwrap()
}

pub fn get_ctf_from_name(name: String) -> Result<ctf::Ctf> {
    let conn = get_conn();
    let mut stmt = conn.prepare("SELECT path, name, url, creds, start, end FROM ctf WHERE name = ?1")?;
    let ctf_iter = stmt.query_map(params![name], |row| {
        Ok(ctf::Ctf::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            match row.get::<usize, String>(3) {
                Ok(creds) => {
                    let creds = creds.split(":").collect::<Vec<&str>>();
                    (creds[0].to_string(), creds[1].to_string())
                }
                Err(_) => ("lorem".to_string(), "ipsum".to_string()),
            },
            row.get::<usize, String>(4)?.parse().unwrap(),
            row.get::<usize, String>(5)?.parse().unwrap(),
        ))
    })?;

    for ctf in ctf_iter {
        return ctf;
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_challenge_from_name(name: String) -> Result<ctf::challenge::Challenge> {
    let conn = get_conn();
    let mut stmt = conn.prepare("SELECT name, category, flag FROM challenge WHERE name = ?1")?;
    let challenge_iter = stmt.query_map(params![name], |row| {
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

pub fn get_all_ctfs() -> Result<Vec<ctf::Ctf>> {
    let conn = get_conn();
    let mut stmt = conn.prepare("SELECT path, name, url, creds, start, end FROM ctf")?;
    let ctf_iter = stmt.query_map(params![], |row| {
        Ok(ctf::Ctf::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            match row.get::<usize, String>(3) {
                Ok(creds) => {
                    let creds = creds.split(":").collect::<Vec<&str>>();
                    (creds[0].to_string(), creds[1].to_string())
                }
                Err(_) => ("lorem".to_string(), "ipsum".to_string()),
            },
            row.get::<usize, String>(4)?.parse().unwrap(),
            row.get::<usize, String>(5)?.parse().unwrap(),
        ))
    })?;

    let mut ctfs = Vec::new();
    for ctf in ctf_iter {
        ctfs.push(ctf.unwrap());
    }

    Ok(ctfs)
}

pub fn chall_exists(ctf_name: &String, chall_name: &String) -> bool {
    let conn = get_conn();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1) AND name = ?2").unwrap();
    let count: i32 = stmt.query_row(params![ctf_name, chall_name], |row| row.get(0)).unwrap();
    count > 0
}
