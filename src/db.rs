use crate::ctf;
use crate::settings;
use rusqlite::{params, Connection, Result};

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
            end TEXT NOT NULL,
            archived INTEGER DEFAULT 0
        )",
        params![],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS challenge (
            id INTEGER PRIMARY KEY,
            ctf_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            flag TEXT NOT NULL,
            archived INTEGER DEFAULT 0,
            FOREIGN KEY(ctf_id) REFERENCES ctf(id)
        )",
        params![],
    )
    .unwrap();

    // create archived column if not exists
    // conn.execute(
    //     "ALTER TABLE ctf ADD COLUMN archived INTEGER DEFAULT 0",
    //     params![],
    // ).unwrap();
    // conn.execute(
    //     "ALTER TABLE challenge ADD COLUMN archived INTEGER DEFAULT 0",
    //     params![],
    // ).unwrap();
    Ok(())
}

pub fn get_conn() -> Connection {
    let db_file = settings::SETTINGS.lock().unwrap().db_file.clone();
    Connection::open(db_file).unwrap()
}

pub fn get_ctf_from_name(conn: &Connection, name: &String, archived: bool) -> Result<ctf::Ctf> {
    let mut stmt = conn.prepare(
        "SELECT path, name, url, creds, start, end FROM ctf WHERE name = ?1 AND archived = ?2",
    )?;
    let ctf_iter = stmt.query_map(params![name, archived], |row| {
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

pub fn get_challenge_from_name(
    conn: &Connection,
    name: String,
) -> Result<ctf::challenge::Challenge, String> {
    let stmt = conn.prepare("SELECT name, category, flag FROM challenge WHERE name = ?1");
    if stmt.is_err() {
        return Err("Error preparing statement".to_string());
    }
    let mut stmt = stmt.unwrap();

    let challenge_iter = stmt.query_map(params![name], |row| {
        Ok(ctf::challenge::Challenge::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    });
    if challenge_iter.is_err() {
        return Err("Error querying challenge".to_string());
    }
    let challenge_iter = challenge_iter.unwrap();

    for challenge in challenge_iter {
        if challenge.is_err() {
            return Err("No challenge found".to_string());
        }
        let aux = challenge.unwrap();
        if is_chall_archived(conn, &name, &aux.name).unwrap_or_else(|_| false) {
            return Err(format!("Challenge {} is archived", name));
        }
        return Ok(aux);
    }

    Err("Challenge not found".to_string())
}

pub fn get_ctf_name_from_challenge(conn: &Connection, name: String) -> Result<String, String> {
    let stmt = conn.prepare("SELECT ctf.name FROM ctf JOIN challenge ON ctf.id = challenge.ctf_id WHERE challenge.name = ?1");
    if stmt.is_err() {
        return Err("Error preparing statement".to_string());
    }
    let mut stmt = stmt.unwrap();
    let ctf_name: Result<String> = stmt.query_row(params![name], |row| row.get(0));
    if ctf_name.is_err() {
        return Err("Challenge not found".to_string());
    }
    let ctf_name = ctf_name.unwrap();
    if is_ctf_archived(conn, &ctf_name).unwrap() {
        return Err(format!("Chall {} is archived under {}", name, ctf_name));
    }

    Ok(ctf_name)
}

pub fn get_all_ctfs(conn: &Connection, archived: bool) -> Result<Vec<ctf::Ctf>> {
    let mut stmt =
        conn.prepare("SELECT path, name, url, creds, start, end FROM ctf WHERE archived = ?1")?;
    let ctf_iter = stmt.query_map(params![archived], |row| {
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
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1 AND archived = 0) AND name = ?2").unwrap();
    let count: i32 = stmt
        .query_row(params![ctf_name, chall_name], |row| row.get(0))
        .unwrap();
    count
}

pub fn ctf_exists(conn: &Connection, name: &String) -> Result<i32, String> {
    let stmt = conn.prepare("SELECT COUNT(*) FROM ctf WHERE name = ?1");
    if stmt.is_err() {
        return Err("Error preparing statement".to_string());
    }
    let mut stmt = stmt.unwrap();

    let count: i32 = stmt.query_row(params![name], |row| row.get(0)).unwrap();
    if count > 0 {
        if is_ctf_archived(conn, name).unwrap() {
            return Err(format!("CTF {} is archived", name));
        }
        let mut stmt = conn.prepare("SELECT id FROM ctf WHERE name = ?1").unwrap();
        let id: i32 = stmt.query_row(params![name], |row| row.get(0)).unwrap();
        return Ok(id);
    }
    Err("CTF not found".to_string())
}

pub fn count_solved_and_total(conn: &Connection, ctf_name: &String) -> (i32, i32) {
    // counts the number of solved challenges in the current context
    let mut stmt = conn
        .prepare(
            "SELECT COUNT(*) FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1)",
        )
        .unwrap();
    let total: i32 = stmt.query_row(params![ctf_name], |row| row.get(0)).unwrap();

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1) AND length(flag) > 0").unwrap();

    let solved: i32 = stmt.query_row(params![ctf_name], |row| row.get(0)).unwrap();

    (solved, total)
}

pub fn remove_ctf(conn: &Connection, name: &String) {
    // remove ctf and all challenges
    conn.execute(
        "DELETE FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1)",
        params![name],
    )
    .unwrap();
    conn.execute("DELETE FROM ctf WHERE name = ?1", params![name])
        .unwrap();
}

pub fn remove_challenge(conn: &Connection, ctf_name: &String, chall_name: &String) {
    conn.execute(
        "DELETE FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1) AND name = ?2",
        params![ctf_name, chall_name],
    )
    .unwrap();
}

pub fn set_chall_flag(conn: &Connection, ctf_name: &String, chall_name: &String, flag: &String) {
    conn.execute("UPDATE challenge SET flag = ?1 WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?2) AND name = ?3", params![flag, ctf_name, chall_name]).unwrap();
}

pub fn is_ctf_archived(conn: &Connection, name: &String) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT archived FROM ctf WHERE name = ?1")?;
    let archived: i32 = stmt.query_row(params![name], |row| row.get(0))?;
    Ok(archived == 1)
}

pub fn is_chall_archived(
    conn: &Connection,
    ctf_name: &String,
    chall_name: &String,
) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT archived FROM challenge WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?1) AND name = ?2")?;
    let archived: i32 = stmt.query_row(params![ctf_name, chall_name], |row| row.get(0))?;
    Ok(archived == 1)
}

pub fn archive_ctf(conn: &Connection, name: &String, _flag: bool) {
    conn.execute(
        "UPDATE ctf SET archived = ?1 WHERE name = ?2",
        params![_flag, name],
    )
    .unwrap();
    conn.execute(
        "UPDATE challenge SET archived = ?1 WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?2)",
        params![_flag, name],
    )
    .unwrap(); // archive all challenges
}

pub fn move_challenge(
    conn: &Connection,
    chall_name: &String,
    old_ctf_name: &String,
    new_ctf_name: &String,
) -> Result<(), String> {
    if ctf_exists(conn, new_ctf_name).is_err() {
        return Err("CTF does not exist".to_string());
    }

    conn.execute("UPDATE challenge SET ctf_id = (SELECT id FROM ctf WHERE name = ?1) WHERE ctf_id = (SELECT id FROM ctf WHERE name = ?2) AND name = ?3", params![new_ctf_name, old_ctf_name, chall_name]).unwrap();

    Ok(())
}
