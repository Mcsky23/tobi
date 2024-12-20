use std::fs;
use std::io::Write;
use std::path::Path;

use crate::ctf::{challenge, Ctf};
use crate::db;
use crate::util::progress_bar;
use colored::Colorize;

use crate::settings;

pub fn get_context() -> (Option<Ctf>, Option<challenge::Challenge>) { 
    let context_file = settings::SETTINGS.lock().unwrap().context_file.clone();
    
    let buf = fs::read_to_string(context_file).unwrap_or_else(|_| {
        println!("No context file found. Create a new CTF!");
        std::process::exit(1);
    });
    let aux = buf.trim().split(":").collect::<Vec<&str>>();

    let ctf_name = aux[0];
    let challenge_name = aux[1];

    let conn = db::get_conn();

    let mut rez = (None, None);
    if ctf_name.len() > 0 {
        let ctf = db::get_ctf_from_name(&conn, &ctf_name.to_string(), false).unwrap_or_else(|e| {
            println!("{}. Context file may be corrupted!", e);
            println!("Resetting context file");
            save_context(None, None);
            std::process::exit(1);
        });
        rez.0 = Some(ctf);
    }

    if challenge_name.len() > 0 {
        let challenge = db::get_challenge_from_name(&conn, challenge_name.to_string()).unwrap_or_else(|_| {
            println!("Challenge {} not found. Context file may be corrupted", challenge_name);
            std::process::exit(1);
        });
        rez.1 = Some(challenge);
    }
    rez
    
}

pub fn save_context(ctf_name: Option<&String>, chall_name: Option<&String>) {
    let context_file = settings::SETTINGS.lock().unwrap().context_file.clone();

    let mut context = String::new();
    match ctf_name {
        Some(ctf_name) => {
            context.push_str(&ctf_name);
        }
        None => {
            context.push_str("");
        }
    }
    context.push_str(":");

    match chall_name {
        Some(chall_name) => {
            context.push_str(&chall_name);
        }
        None => {
            context.push_str("");
        }
    }

    let mut file = fs::File::create(context_file).unwrap();
    file.write_all(context.as_bytes()).unwrap();
}

pub fn switch_context(ctf_name: &String, chall_name: Option<&String>, _show_context: bool) {
    let conn = db::get_conn();
    if let Err(e) = db::get_ctf_from_name(&conn, ctf_name, false) {
        println!("Error: {}", e);
        return;
    }

    match chall_name {
        Some(chall_name) => {
            if db::chall_exists(&conn, ctf_name, chall_name) == 0 {
                println!("Challenge not found in {}", ctf_name);
                return;
            }
        },
        None => {}
    }
    save_context(Some(ctf_name), chall_name);
    if _show_context {
        show_context();
    }
}

pub fn show_context() {
    let (ctf, chall) = get_context();
    match ctf {
        Some(ctf) => {
            match chall {
                Some(chall) => {
                    println!("Currently working on {} {} {}", ctf.metadata.name.bold(), "➜".green(), chall);
                },
                None => {
                    println!("Currently working on {}", ctf.metadata.name.bold());
                }
            };
            let conn = db::get_conn();
            let (solved, total) = db::count_solved_and_total(&conn, &ctf.metadata.name);
            println!("Solved {}/{} {}", solved, total, progress_bar(solved as usize, total as usize));
        },
        None => {
            println!("Currently working on nothing.");
        }
    }
}

pub fn change_directory() {
    // get context
    let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();

    let (ctf, chall) = get_context();
    match ctf {
        Some(ctf) => {
            match chall {
                Some(chall) => {
                    let category_dir = format!("{}/{}/{}", workdir, ctf.metadata.name, chall.category);
                    let chall_dir = format!("{}/{}", category_dir, chall.name);
                    let chall_path = Path::new(&chall_dir);
                    if !chall_path.exists() {
                        println!("Challenge directory not found.") // this code should never be reached based on other checks
                    } else {
                        // run system command to change directory
                        println!("^CHANGE_DIR^{}^CHANGE_DIR^", chall_dir);
                    }
                },
                None => {
                    let ctf_dir = ctf.file_path;
                    let ctf_path = Path::new(&ctf_dir);
                    if !ctf_path.exists() {
                        println!("CTF directory not found.") // this code should never be reached based on other checks
                    } else {
                        // change directory by outputting the path and using a shell script
                        println!("^CHANGE_DIR^{}^CHANGE_DIR^", ctf_dir);
                    }
                }
            }
        },
        None => {
            println!("No context found.");
        }
    }
}
