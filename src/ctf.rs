use chrono::{DateTime, Utc};
use std::fs;
use rusqlite::{Connection, params};
use crate::db;
use crate::context;

use crate::db::ctf_exists;
use crate::settings;

pub mod challenge;

pub struct Meta {
    pub name: String,
    url: String,
    creds: (String, String),
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

pub struct Ctf {
    pub file_path: String, 
    pub metadata: Meta,
    pub challenges: Vec<challenge::Challenge>,
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

    pub fn add_challenge(&mut self, challenge: challenge::Challenge) {
        self.challenges.push(challenge);
    }

    pub fn save_to_db(&self) {
        let conn: Connection = db::get_conn();
        if let Some(_) = ctf_exists(&conn, &self.metadata.name) {
            // update ctf
            conn.execute(
                "UPDATE ctf SET url = ?1, creds = ?2, start = ?3, end = ?4 WHERE name = ?5",
                params![self.metadata.url, format!("{}:{}", self.metadata.creds.0, self.metadata.creds.1), self.metadata.start.to_rfc3339(), self.metadata.end.to_rfc3339(), self.metadata.name],
            ).unwrap();
        } else {
            // insert ctf
            conn.execute(
                "INSERT INTO ctf (path, name, url, creds, start, end) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![self.file_path, self.metadata.name, self.metadata.url, format!("{}:{}", self.metadata.creds.0, self.metadata.creds.1), self.metadata.start.to_rfc3339(), self.metadata.end.to_rfc3339()],
            ).unwrap();
        }
    }

    pub fn print_challs(&self) {
        if self.challenges.len() == 0 {
            println!("No challenges found in {}", self.metadata.name);
            return;
        }
        println!("{}", self.metadata.name);
        for challenge in self.challenges.iter() {
            println!("  {} [{}] {}", if challenge.flag.len() > 0 { "✓" } else { " " }, challenge.category, challenge.name);
        }
    }
}

pub fn quick_new(name: String) {
    let file_path = settings::WORKDIR.to_string() + "/" + &name;
    match fs::create_dir(&file_path) {
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
    ctf.save_to_db();
    // update context
    crate::context::save_context(Some(&ctf.metadata.name), None);

}

pub fn new_challenge(name: String, category: String) {
    match challenge::check_type(category.as_str()) {
        Some(chall_type) => {
            println!("Creating new challenge {} of type {}", name, chall_type);
        },
        None => {
            println!("Invalid challenge type");
            std::process::exit(1);
        }
    };

    let (ctf, _) = crate::context::get_context();
    let ctf = match ctf {
        Some(ctf) => ctf,
        None => {
            println!("You are not working on any CTF");
            std::process::exit(1);
        }
    };

    let conn = db::get_conn();
    // check if challenge already exists in current CTF
    if db::chall_exists(&conn, &ctf.metadata.name, &name) {
        println!("Challenge already exists in current CTF");
        std::process::exit(1);
    }

    let challenge = challenge::Challenge::new(name, category, "".to_string());
    challenge.create_file(&ctf.metadata.name);

    challenge.save_to_db(&ctf.metadata.name);

    context::save_context(Some(&ctf.metadata.name), Some(&challenge.name));

}

