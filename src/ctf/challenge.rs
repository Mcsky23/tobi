use crate::context;
use crate::db;
use crate::{db::get_ctf_name_from_challenge, settings};
use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;

pub struct Challenge {
    pub name: String,
    pub category: ChallengeType,
    pub flag: String,
}

pub enum ChallengeType {
    Web,
    Pwn,
    Crypto,
    Forensics,
    Reversing,
    Misc,
}

impl ChallengeType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "web" => ChallengeType::Web,
            "pwn" => ChallengeType::Pwn,
            "crypto" => ChallengeType::Crypto,
            "forensics" => ChallengeType::Forensics,
            "reversing" => ChallengeType::Reversing,
            "misc" => ChallengeType::Misc,
            _ => ChallengeType::Misc,
        }
    }
}

impl std::fmt::Display for ChallengeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChallengeType::Web => write!(f, "web"),
            ChallengeType::Pwn => write!(f, "pwn"),
            ChallengeType::Crypto => write!(f, "crypto"),
            ChallengeType::Forensics => write!(f, "forensics"),
            ChallengeType::Reversing => write!(f, "reversing"),
            ChallengeType::Misc => write!(f, "misc"),
        }
    }
}

impl Challenge {
    pub fn new(name: String, category: String, flag: String) -> Self {
        Challenge {
            name,
            category: match category.as_str() {
                "web" => ChallengeType::Web,
                "pwn" => ChallengeType::Pwn,
                "crypto" => ChallengeType::Crypto,
                "forensics" => ChallengeType::Forensics,
                "reversing" => ChallengeType::Reversing,
                "misc" => ChallengeType::Misc,
                _ => ChallengeType::Misc,
            },
            flag: flag,
        }
    }

    pub fn create_file(&self, ctf_name: &str) {
        // check if category directory exists
        let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();

        let category_dir = format!("{}/{}/{}", workdir, ctf_name, self.category);
        if !Path::new(&category_dir).exists() {
            fs::create_dir(&category_dir).unwrap();
        }

        let chall_dir = format!("{}/{}", category_dir, self.name);
        fs::create_dir(&chall_dir).unwrap();
    }

    pub fn save_to_db(&self, ctf_name: &String) {
        let conn: Connection = db::get_conn();
        let ctf_id = db::ctf_exists(&conn, ctf_name);
        if let Err(e) = ctf_id {
            println!("{}", e);
            std::process::exit(1);
        }
        let ctf_id = ctf_id.unwrap();
        if db::chall_exists(&conn, ctf_name, &self.name) == 0 {
            conn.execute(
                "INSERT INTO challenge (ctf_id, name, category, flag) VALUES (?1, ?2, ?3, ?4)",
                params![ctf_id, self.name, self.category.to_string(), self.flag],
            )
            .unwrap();
        } else {
            // update challenge
            conn.execute(
                "UPDATE challenge SET flag = ?1 WHERE ctf_id = ?2 AND name = ?3",
                params![self.flag, ctf_id, self.name],
            )
            .unwrap();
        }
    }

    fn change_chall_dir(&self, ctf_name: &String, category: &String, name: &String) -> String {
        let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();
        let old_path = format!("{}/{}/{}/{}", workdir, ctf_name, self.category, self.name);
        let new_path = format!("{}/{}/{}/{}", workdir, ctf_name, category, name);
        // check if category directory exists
        if !Path::new(&format!("{}/{}/{}", workdir, ctf_name, category)).exists() {
            fs::create_dir(&format!("{}/{}/{}", workdir, ctf_name, category)).unwrap();
        }

        fs::rename(old_path, &new_path).unwrap();
        new_path
    }

    pub fn edit_chall(&mut self, name: &String, category: &String) {
        // check if name is unique
        let conn: Connection = db::get_conn();
        let ctf_name = get_ctf_name_from_challenge(&conn, self.name.clone()).unwrap();
        if db::chall_exists(&conn, &ctf_name, &name) > 1 {
            println!("Challenge with name {} already exists", name);
            std::process::exit(1);
        }

        let new_path = self.change_chall_dir(&ctf_name, category, name);

        // update name in db
        conn.execute(
            "UPDATE challenge SET name = ?1, category = ?2 WHERE name = ?3",
            params![name, category, self.name],
        )
        .unwrap();

        self.name = name.clone();
        self.category = ChallengeType::from_str(category.as_str());
        println!("CHANGE_DIR: {}", new_path);
    }
}

pub fn check_type(chall_type: &str) -> Option<&str> {
    match chall_type {
        "web" => Some("web"),
        "pwn" => Some("pwn"),
        "crypto" => Some("crypto"),
        "forensics" => Some("forensics"),
        "reversing" => Some("reversing"),
        "misc" => Some("misc"),
        _ => None,
    }
}

pub fn remove_chall(ctf_name: &String, chall_name: &String) {
    let conn = db::get_conn();
    let chall_category = db::get_challenge_from_name(&conn, chall_name.to_string())
        .unwrap()
        .category
        .to_string();

    db::remove_challenge(&conn, &ctf_name, &chall_name.to_string());
    context::save_context(Some(&ctf_name), None);

    // remove challenge directory
    let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();
    let file_path = format!("{}/{}/{}/{}", workdir, ctf_name, chall_category, chall_name);
    fs::remove_dir_all(file_path).unwrap();
    println!("Removed challenge {} from CTF {}", chall_name, ctf_name);
    // TODO: switch back to ctf context(done)
}
