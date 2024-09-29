use rusqlite::{Connection, Result, params};
use crate::ctf;
use crate::settings;

pub fn init_db() -> Result<(), rusqlite::Error> {
    // Create a new SQLite database
    let conn = get_conn();
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
    Ok(())

}

pub fn get_conn() -> Connection {
    let db_file = settings::SETTINGS.lock().unwrap().db_file.clone();
    Connection::open(db_file).unwrap()
}

pub fn get_ctf_from_name(conn: &Connection, name: &String) -> Result<ctf::Ctf> {
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
        // populate challenges
        let mut ctf_buf = ctf.unwrap();
        let mut stmt = conn.prepare("SELECT name, category, flag FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1)")?;
        let challenge_iter = stmt.query_map(params![name], |row| {
            Ok(ctf::challenge::Challenge::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ))
        })?;

        for challenge in challenge_iter {
            ctf_buf.add_challenge(challenge.unwrap());
        }
        return Ok(ctf_buf);
    }

    Err(rusqlite::Error::QueryReturnedNoRows)
}

pub fn get_challenge_from_name(conn: &Connection, name: String) -> Result<ctf::challenge::Challenge> {

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

pub fn get_ctf_name_from_challenge(conn: &Connection, name: String) -> Result<String> {

    let mut stmt = conn.prepare("SELECT ctf.name FROM ctf JOIN challenge ON ctf.id = challenge.ctf_id WHERE challenge.name = ?1")?;
    let ctf_name: String = stmt.query_row(params![name], |row| row.get(0))?;
    Ok(ctf_name)
}

pub fn get_all_ctfs(conn: &Connection) -> Result<Vec<ctf::Ctf>> {

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
        // populate challenges
        let mut ctf_buf = ctf.unwrap();
        let mut stmt = conn.prepare("SELECT name, category, flag FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1)")?;
        let challenge_iter = stmt.query_map(params![ctf_buf.metadata.name], |row| {
            Ok(ctf::challenge::Challenge::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ))
        })?;

        for challenge in challenge_iter {
            ctf_buf.add_challenge(challenge.unwrap());
        }
        ctfs.push(ctf_buf);
    }

    Ok(ctfs)
}

pub fn chall_exists(conn: &Connection, ctf_name: &String, chall_name: &String) -> i32 {

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1) AND name = ?2").unwrap();
    let count: i32 = stmt.query_row(params![ctf_name, chall_name], |row| row.get(0)).unwrap();
    count
}

pub fn ctf_exists(conn: &Connection, name: &String) -> Option<i32> {

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM ctf WHERE name = ?1").unwrap();
    let count: i32 = stmt.query_row(params![name], |row| row.get(0)).unwrap();
    if count > 0 {
        let mut stmt = conn.prepare("SELECT id FROM ctf WHERE name = ?1").unwrap();
        let id: i32 = stmt.query_row(params![name], |row| row.get(0)).unwrap();
        return Some(id);
    }
    None
}

pub fn count_solved_and_total(conn: &Connection, ctf_name: &String) -> (i32, i32) {
    // counts the number of solved challenges in the current context
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1)").unwrap();
    let total: i32 = stmt.query_row(params![ctf_name], |row| row.get(0)).unwrap();

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1) AND length(flag) > 0").unwrap();

    let solved: i32 = stmt.query_row(params![ctf_name], |row| row.get(0)).unwrap();

    (solved, total)
}

pub fn remove_ctf(conn: &Connection, name: &String) {
    // remove ctf and all challenges
    conn.execute("DELETE FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1)", params![name]).unwrap();
    conn.execute("DELETE FROM ctf WHERE name = ?1", params![name]).unwrap();
}

pub fn remove_challenge(conn: &Connection, ctf_name: &String, chall_name: &String) {
    conn.execute("DELETE FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1) AND name = ?2", params![ctf_name, chall_name]).unwrap();
}

pub fn set_chall_flag(conn: &Connection, ctf_name: &String, chall_name: &String, flag: &String) {
    conn.execute("UPDATE challenge SET flag = ?1 WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?2) AND name = ?3", params![flag, ctf_name, chall_name]).unwrap();
}