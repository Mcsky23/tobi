use crate::settings;
use std::path::Path;
use std::fs;
use crate::db;
use rusqlite::{Connection, params};

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
        let category_dir = format!("{}/{}/{}", settings::WORKDIR, ctf_name, self.category);
        if !Path::new(&category_dir).exists() {
            fs::create_dir(&category_dir).unwrap();
        }

        let chall_dir = format!("{}/{}", category_dir, self.name);
        fs::create_dir(&chall_dir).unwrap();
    }

    pub fn save_to_db(&self, ctf_name: &String, ctf_id: i32) {
        if !db::chall_exists(ctf_name, &self.name) {
            let conn: Connection = db::get_conn();
            conn.execute("INSERT INTO challenge (ctf_id, name, category, flag) VALUES (?1, ?2, ?3, ?4)", params![ctf_id, self.name, self.category.to_string(), self.flag]).unwrap();
        } else {
            // update challenge
            let conn = db::get_conn();
            conn.execute("UPDATE challenge SET flag = ?1 WHERE ctf_id = ?2 AND name = ?3", params![self.flag, ctf_id, self.name]).unwrap();
        }
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