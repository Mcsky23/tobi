use chrono::{DateTime, Utc};
use std::fs;
use rusqlite::{Connection, Result, params};

use crate::settings;

pub mod challenge;

struct Meta {
    name: String,
    url: String,
    creds: (String, String),
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

pub struct Ctf {
    file_path: String, 
    pub metadata: Meta,
    challenges: Vec<challenge::Challenge>,
}

impl Ctf {
    pub fn new(file_path: String, name: String, url: String, creds: (String, String), start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Ctf {
            file_path,
            metadata: Meta {
                name,
                url,
                creds,
                start,
                end,
            },
            challenges: Vec::new(),
        }
    }
}

pub fn quick_new(name: String) {
    let file_path = settings::WORKDIR.to_string() + "/" + &name;
    match(fs::create_dir(&file_path)) {
        Ok(_) => {
            println!("Created new CTF at {}", file_path);
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    println!("CTF already exists at {}", file_path);
                    return;
                }
                _ => {
                    println!("Error creating CTF: {}", e);
                }
            }
        }
    }
    let ctf = Ctf::new(file_path, name, "".to_string(), ("".to_string(), "".to_string()), Utc::now(), Utc::now());
    let conn = crate::db::get_conn();
    conn.execute(
        "INSERT INTO ctf (path, name, url, creds, start, end) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![ctf.file_path, ctf.metadata.name, ctf.metadata.url, format!("{}:{}", ctf.metadata.creds.0, ctf.metadata.creds.1), ctf.metadata.start.to_rfc3339(), ctf.metadata.end.to_rfc3339()],
    ).unwrap();
}