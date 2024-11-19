use chrono::{DateTime, Utc};
use std::fs;
use rusqlite::{Connection, params};
use crate::db;
use crate::context;
use fs_extra::dir::get_size;
use humansize::{format_size, DECIMAL};
use colored::Colorize;

use crate::db::{ctf_exists, count_solved_and_total};
use crate::util::progress_bar;
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
        match ctf_exists(&conn, &self.metadata.name) {
            Ok(_) => {
                // update ctf
                conn.execute(
                    "UPDATE ctf SET url = ?1, creds = ?2, start = ?3, end = ?4 WHERE name = ?5",
                    params![self.metadata.url, format!("{}:{}", self.metadata.creds.0, self.metadata.creds.1), self.metadata.start.to_rfc3339(), self.metadata.end.to_rfc3339(), self.metadata.name],
                ).unwrap();
            },
            Err(e) => {
                if e.ends_with("is archived") {
                    println!("CTF {} is archived. Cannot update", self.metadata.name);
                    return;
                } else {
                    conn.execute(
                        "INSERT INTO ctf (path, name, url, creds, start, end) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        params![self.file_path, self.metadata.name, self.metadata.url, format!("{}:{}", self.metadata.creds.0, self.metadata.creds.1), self.metadata.start.to_rfc3339(), self.metadata.end.to_rfc3339()],
                    ).unwrap();
                }   
            }
        }
    }

    pub fn print_challs(&self, with_flags: bool) {
        if self.challenges.len() == 0 {
            println!("No challenges found in {}", self.metadata.name);
            return;
        }
        let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();
        let ctfdir = format!("{}/{}", workdir, self.metadata.name);
        let ctf_size = get_size(&ctfdir).unwrap_or(0);
        let (solved, total) = count_solved_and_total(&crate::db::get_conn(), &self.metadata.name);
        let progress_bar = progress_bar(solved as usize, total as usize);
        println!("{}{} - {}\n  {}", "➜".green(), self.metadata.name.bold(), format_size(ctf_size, DECIMAL), progress_bar);
        for challenge in self.challenges.iter() {
            print!("  {} {}", if challenge.flag.len() > 0 { "✓".bold().blue() } else { " ".bold() }, challenge);
            if with_flags {
                println!("{} {}", " ".repeat(40 - challenge.name.len()), challenge.flag);
            } else {
                println!();
            }
        }
    }

    pub fn change_name(&mut self, new_name: String) {
        let conn = db::get_conn();
        
        
        // change directory name
        let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();
        let old_path = format!("{}/{}", workdir, self.metadata.name);
        let new_path = format!("{}/{}", workdir, new_name);
        fs::rename(old_path, &new_path).unwrap();

        conn.execute("UPDATE ctf SET name = ?1, path = ?2 WHERE name = ?3", params![new_name, new_path, self.metadata.name]).unwrap();
        
        self.metadata.name = new_name;

        println!("^CHANGE_DIR^{}^CHANGE_DIR^", new_path);
    }

    pub fn remove_ctf(&self, archived: bool) {
        let conn = db::get_conn();
        let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();
        let name = &self.metadata.name;
        // remove ctf from db
        db::remove_ctf(&conn, name);
        context::save_context(None, None);

        // remove ctf directory
        match archived {
            false => {
                let file_path = workdir + "/" + name;
                fs::remove_dir_all(file_path).unwrap();
                println!("Removed CTF {}", name);
            }
            true => {
                let file_path = format!("{}/.archived/{}.tar.bz2", workdir, name);
                fs::remove_file(file_path).unwrap();
                println!("Removed [archived] CTF {}", name);
            }
        }
    }

    pub fn zip_ctf_dir(&self) {
        // zip the ctf directory
        let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();
        let name = &self.metadata.name;
        let ctfdir = format!("{}/{}", workdir, name);
        let orig_size = get_size(&ctfdir).unwrap();
        let zip_path = format!("{}/.archived/{}.tar.bz2", workdir, name);
        // if .archived does not exist, create it
        let archived_dir = format!("{}/.archived", workdir);
        match fs::create_dir(&archived_dir) {
            Ok(_) => {},
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::AlreadyExists => {},
                    _ => {
                        println!("Error creating .archived directory: {}", e);
                        return;
                    }
                }
            }
        }
        // use tar
        let cmd = format!("tar -cjvf {} -C {} .", zip_path, ctfdir);
        // println!("[+] Command: {}", cmd);
        let mut _output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .spawn()
            .expect(format!("Failed to execute command: {}", cmd).as_str());
        
        _output.wait().unwrap();
        println!("{} Command ran successfully", "+".green());
        fs::remove_dir_all(ctfdir).unwrap();
        println!("{} Removed zip file", "+".green());
        let new_size = get_size(&zip_path).unwrap();
        println!("{} Size reduced from {} to {}", "!".bright_red(), format_size(orig_size, DECIMAL), format_size(new_size, DECIMAL));
    }

    pub fn unzip_ctf_dir(&self) {
        // unzip the ctf directory
        let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();
        let name = &self.metadata.name;
        let ctfdir = format!("{}/{}", workdir, name);
        let zip_path = format!("{}/.archived/{}.tar.bz2", workdir, name);
        let orig_size = get_size(&zip_path).unwrap();
        // create ctfdir path
        match fs::create_dir(&ctfdir) {
            Ok(_) => {},
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::AlreadyExists => {},
                    _ => {
                        println!("Error creating ctf directory: {}", e);
                        return;
                    }
                }
            }
        }
        // use tar
        let cmd = format!("tar -xjvf {} -C {}", zip_path, ctfdir);
        // println!("[+] Running command: {}", cmd);
        let mut _output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .spawn()
            .expect(format!("Failed to execute command: {}", cmd).as_str());

        _output.wait().unwrap();
        // remove zip file
        println!("{} Command ran successfully", "+".green());
        fs::remove_file(zip_path).unwrap();
        println!("{} Removed zip file", "+".green());
        let new_size = get_size(&ctfdir).unwrap();
        println!("{} Size inflated from {} to {}", "!".bright_red(), format_size(orig_size, DECIMAL), format_size(new_size, DECIMAL));

    }

    pub fn archive(&self) {
        self.zip_ctf_dir();
        let conn = db::get_conn();
        db::archive_ctf(&conn, &self.metadata.name, true);
    }

    pub fn unarchive(&self) {
        self.unzip_ctf_dir();
        let conn = db::get_conn();
        db::archive_ctf(&conn, &self.metadata.name, false);
    }

}

pub fn quick_new(name: String) {
    let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();
    let file_path = workdir + "/" + &name;
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
                    println!("{}Error creating CTF: {}", "✗".bright_red().bold(), e);
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
    let (ctf, _) = crate::context::get_context();
    let ctf = match ctf {
        Some(ctf) => ctf,
        None => {
            println!("{}You are not working on any CTF", "✗".bright_red().bold());
            std::process::exit(1);
        }
    };

    match challenge::check_type(category.as_str()) {
        None => {
            println!("{}Invalid challenge type", "✗".bright_red().bold());
            std::process::exit(1);
        }
        _ => {}
    };

    let conn = db::get_conn();
    // check if challenge already exists in current CTF
    if db::chall_exists(&conn, &ctf.metadata.name, &name) > 0 {
        println!("{}Challenge already exists in current CTF", "✗".bright_red().bold());
        std::process::exit(1);
    }
    
    let challenge = challenge::Challenge::new(name, category, "".to_string());
    println!("Creating new challenge {}", challenge);
    challenge.create_file(&ctf.metadata.name);
    challenge.save_to_db(&ctf.metadata.name);

    context::save_context(Some(&ctf.metadata.name), Some(&challenge.name));

}

