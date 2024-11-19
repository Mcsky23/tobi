use crate::autocompletions;
use crate::context;
use crate::ctf;
use crate::ctf::challenge;
use crate::ctf::challenge::remove_chall;
use crate::db;
use crate::db::is_ctf_archived;
use crate::help;
use crate::settings::{self, SETTINGS};
use crate::undo::{undo, UndoAction};
use crate::util::are_you_sure;
use colored::Colorize;

trait ArgName<T> {
    fn validate(&self) -> &T;
}

impl ArgName<String> for String {
    fn validate(&self) -> &String {
        // check that characters are alphanumeric or underscore
        if self.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return &self;
        }
        println!("{}Invalid name in argument: {}", "✗".bright_red().bold() ,self);
        std::process::exit(1);
    }
}

pub fn do_action(mut args: Vec<String>) {
    if args.len() == 1 {
        let tobi_command = SETTINGS.lock().unwrap().tobi_command.clone();
        args.push(tobi_command);
    }
    let action = args[1].validate().as_str();
    match action {
        "help" => {
            help::print_help();
        }
        "ctf" => {
            match args.len() {
                2 => {
                    UndoAction::new_dir_change().log_action();
                    context::change_directory();
                    context::show_context();
                }
                3 => {
                    // change directory to specified ctf but don't change the context
                    // check if ctf exists
                    let ctf_name = args[2].validate();
                    if ctf_name == "NO_UNDO" {
                        context::change_directory();
                        context::show_context();
                        return;
                    }
                    let conn = db::get_conn();
                    match db::get_ctf_from_name(&conn, &ctf_name, false) {
                        Ok(ctf) => {
                            UndoAction::new_dir_change().log_action();
                            println!("^CHANGE_DIR^{}^CHANGE_DIR^", ctf.file_path);
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }
                4 => {
                    // change directory to specified ctf and challenge
                    let ctf_name = args[2].validate();
                    let chall_name = args[3].validate();
                    let conn = db::get_conn();

                    match db::get_ctf_from_name(&conn, &ctf_name, false) {
                        Ok(ctf) => {
                            match db::get_challenge_from_name(&conn, chall_name.to_string()) {
                                Ok(chall) => {
                                    UndoAction::new_dir_change().log_action();
                                    println!("^CHANGE_DIR^{}/{}^CHANGE_DIR^", ctf.file_path, chall.category);
                                }
                                Err(_) => {
                                    println!("Challenge not found");
                                }
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }
                _ => {
                    println!("{}Invalid number of arguments", "✗".bright_red().bold());
                    println!("Usage: tobi ctf - change to CTF directory");
                }
            }
        }
        "new" => {
            let what = args[2].validate().as_str();
            match what {
                "ctf" => {
                    if args.len() != 4 {
                        println!("{}Invalid number of arguments", "✗".bright_red().bold());
                        println!("Usage: tobi new ctf <name> - create a new ctf");
                    }
                    let name = args[3].validate();
                    UndoAction::new_ctf_create(name).log_action();
                    ctf::quick_new(name.to_string());
                }
                chall_type => {
                    if args.len() != 4 {
                        println!("{}Invalid number of arguments", "✗".bright_red().bold());
                        println!("Usage: tobi new <type> <name> - create a new challenge");
                    }
                    let name = args[3].validate();
                    if let Ok(true) = is_ctf_archived(&db::get_conn(), &name) {
                        println!("{}Cannot create challenge in [archived] {}", "✗".bright_red().bold(), name);
                        std::process::exit(1);
                    }
                    ctf::new_challenge(name.to_string(), chall_type.to_string());
                    UndoAction::new_chall_create(&name).log_action(); // no need to error check here, if there is no ctf in scope program exits anyways
                }
            }
        }
        "edit" => {
            match args.len() {
                4 => {
                    let category = args[2].validate();
                    let name = args[3].validate();
                    let (ctf, chall) = context::get_context();

                    if category == "ctf" {
                        // edit ctf name
                        if let None = ctf {
                            println!("{}No CTF found in context","✗".bright_red().bold());
                            std::process::exit(1);
                        }
                        let mut ctf = ctf.unwrap();

                        UndoAction::new_ctf_edit(&ctf.metadata.name, &name).log_action();

                        ctf.change_name(name.clone());
                        context::switch_context(&ctf.metadata.name, None, false);
                        println!("Edited CTF {}", &ctf.metadata.name.bold());
                    } else {
                        if let None = chall {
                            println!("{}No challenge found in context", "✗".bright_red().bold());
                            std::process::exit(1);
                        }

                        let mut chall = chall.unwrap();
                        let ctf = ctf.unwrap();
                        if challenge::check_type(&category).is_none() {
                            println!("{}Invalid challenge type", "✗".bright_red().bold());
                            std::process::exit(1);
                        }

                        UndoAction::new_chall_edit(&chall.name, &chall.category.to_string())
                            .log_action();

                        chall.edit_chall(name, category);
                        context::switch_context(&ctf.metadata.name, Some(&name), false);

                        println!(
                            "Edited {} {} {}",
                            &ctf.metadata.name.bold(), "➜".green().bold(), &chall
                        );
                    }
                }
                _ => {
                    println!("{}Invalid number of arguments", "✗".bright_red().bold());
                    println!("Usage: tobi edit <category> <name>");
                    println!("       tobi edit ctf <name>");
                }
            }
        }
        "list" => {
            // For now, just list all ctfs
            match args.len() {
                2 => {
                    // list all challenges in current ctf
                    let (ctf, _) = context::get_context();
                    if let None = ctf {
                        println!("{}No CTF found in context", "✗".bright_red().bold() ,);
                        std::process::exit(1);
                    }
                    let ctf = ctf.unwrap();
                    ctf.print_challs(false);
                }
                3 => {
                    match args[2].as_str() {
                        "all" => {
                            // list all chalenges in all ctfs
                            let ctfs = db::get_all_ctfs(&db::get_conn(), false).unwrap();
                            if ctfs.len() == 0 {
                                println!("No ctfs found");
                            }
                            for ctf in ctfs {
                                ctf.print_challs(false);
                                println!();
                            }

                            let ctfs = db::get_all_ctfs(&db::get_conn(), true).unwrap();
                            if ctfs.len() > 0 {
                                println!();
                                for ctf in ctfs {
                                    println!("{}{} {}", "✗".bright_red(), "[ARCHIVED]".white(), ctf.metadata.name);
                                }
                                println!();
                            }
                            return;
                        }
                        "ctfs" => {
                            // list all ctf names
                            let ctfs = db::get_all_ctfs(&db::get_conn(), false).unwrap();
                            if ctfs.len() == 0 {
                                println!("No ctfs found");
                            }
                            for ctf in ctfs {
                                println!("{}", ctf.metadata.name);
                            }
                        }
                        "flags" => {
                            // list all challenges with flags
                            let (ctf, _) = context::get_context();
                            if let None = ctf {
                                println!("{}No CTF found in context", "✗".bright_red().bold());
                                std::process::exit(1);
                            }
                            let ctf = ctf.unwrap();
                            ctf.print_challs(true);
                        }
                        "archived" => {
                            // list all archived ctfs
                            let ctfs = db::get_all_ctfs(&db::get_conn(), true).unwrap();
                            if ctfs.len() == 0 {
                                println!("No archived ctfs found");
                            }
                            for ctf in ctfs {
                                println!("[ARCHIVED] {}", ctf.metadata.name);
                            }
                        }
                        ctf_name => {
                            let ctf_name = ctf_name.to_string();
                            let ctf_name = ctf_name.validate();
                            let conn = db::get_conn();
                            if let Ok(a) = is_ctf_archived(&conn, &ctf_name) {
                                if a {
                                    print!("[ARCHIVED] ");
                                    let ctf =
                                        db::get_ctf_from_name(&conn, &ctf_name, true).unwrap();
                                    ctf.print_challs(false);
                                } else {
                                    let ctf =
                                        db::get_ctf_from_name(&conn, &ctf_name, false).unwrap();
                                    ctf.print_challs(false);
                                }
                            } else {
                                println!("CTF not found");
                            }
                        }
                    }
                }
                _ => {
                    println!("{}Invalid number of arguments", "✗".bright_red().bold());
                    println!("Usage: tobi list - list all ctfs");
                    println!("       tobi list <ctf> - list all challenges in ctf");
                }
            }
        }
        "rm" => {
            match args.len() {
                3 => {
                    // figure out if this is a ctf or a challenge
                    let anon_name = args[2].validate();
                    let conn = db::get_conn();

                    match are_you_sure(anon_name) {
                        true => {}
                        false => {
                            println!("{}Canceled", "✗".bright_red().bold());
                            std::process::exit(1);
                        }
                    }

                    if let Ok(ctf) = db::get_ctf_from_name(&conn, &anon_name, false) {
                        // remove ctf
                        ctf.remove_ctf(false);
                    } else if let Ok(ctf) = db::get_ctf_from_name(&conn, &anon_name, true) {
                        ctf.remove_ctf(true);
                    } else if db::get_challenge_from_name(&conn, anon_name.to_string()).is_ok() {
                        let ctf_name =
                            db::get_ctf_name_from_challenge(&conn, anon_name.to_string())
                                .unwrap_or_else(|e| {
                                    println!("{}{}", "✗".bright_red().bold(), e);
                                    std::process::exit(1);
                                });
                        if let Ok(true) = is_ctf_archived(&conn, &ctf_name) {
                            println!("{}Cannot remove challenge {} from [archived] {}", "✗".bright_red().bold() , anon_name, ctf_name);
                            std::process::exit(1);
                        }
                        remove_chall(&ctf_name, anon_name);
                    } else {
                        println!("No ctf or challenge found with name {}", anon_name);
                    }
                }
                4 => {
                    // remove challenge
                    let ctf_name = args[2].validate();
                    let chall_name = args[3].validate();

                    match are_you_sure(chall_name) {
                        true => {}
                        false => {
                            println!("{}Canceled", "✗".bright_red().bold());
                            std::process::exit(1);
                        }
                    }

                    remove_chall(ctf_name, chall_name);
                }
                _ => {
                    println!("{}Invalid number of arguments", "✗".bright_red().bold());
                    println!("Usage: tobi rm <ctf/challenge> - remove ctf or challenge");
                    println!("       tobi rm <ctf> <challenge> - remove challenge from ctf");
                }
            }
        }
        "solve" => {
            // solve the current challenge
            let (ctf, challenge) = context::get_context();
            if let None = ctf {
                println!("{}No CTF found in context", "✗".bright_red().bold());
                std::process::exit(1);
            }
            let ctf = ctf.unwrap();
            if args.len() != 3 {
                println!("{}Invalid number of arguments", "✗".bright_red().bold());
                println!("Usage: tobi solve <flag>");
                std::process::exit(1);
            }
            let flag = args[2].to_string();

            match challenge {
                Some(mut challenge) => {
                    // solve challenge
                    challenge.flag = flag;
                    challenge.save_to_db(&ctf.metadata.name);
                    UndoAction::new_chall_solve(&ctf.metadata.name, &challenge.name).log_action();
                    println!(
                        "Solved {} {} {}: {}",
                        &ctf.metadata.name, "➜".green(), &challenge, &challenge.flag
                    );
                }
                None => {
                    println!("{}You are currently not working on a challenge", "✗".bright_red().bold());
                    std::process::exit(1);
                }
            }
        }
        "unsolve" => {
            // remove the flag from the current challenge
            // solve the current challenge
            let (ctf, challenge) = context::get_context();
            if let None = ctf {
                println!("{}No CTF found in context", "✗".bright_red().bold());
                std::process::exit(1);
            }
            let ctf = ctf.unwrap();
            if args.len() != 2 {
                println!("{}Invalid number of arguments", "✗".bright_red().bold());
                println!("Usage: tobi solve <flag>");
                std::process::exit(1);
            }
            match challenge {
                Some(mut challenge) => {
                    // solve challenge
                    UndoAction::new_chall_unsolve(
                        &ctf.metadata.name,
                        &challenge.name,
                        &challenge.flag,
                    )
                    .log_action();
                    challenge.flag = "".to_string();
                    challenge.save_to_db(&ctf.metadata.name);
                    println!(
                        "Unsolved {} {} {}",
                        &ctf.metadata.name, "➜".green(), &challenge
                    );
                }
                None => {
                    println!("{}You are currently not working on a challenge", "✗".bright_red().bold());
                    std::process::exit(1);
                }
            }
        }
        "context" => {
            match args.len() {
                2 => {
                    // show context
                    context::show_context();
                }
                3 => {
                    // set context
                    let anon_name = args[2].validate();
                    let conn = db::get_conn();
                    if let Ok(true) = is_ctf_archived(&conn, &anon_name) {
                        println!("{}Cannot switch context to [archived] {}", "✗".bright_red().bold(), anon_name);
                        std::process::exit(1);
                    }
                    let context_changes_dir = SETTINGS.lock().unwrap().context_changes_dir;
    
                    // figure if this is a ctf or a challenge by searching through db
                    if db::get_ctf_from_name(&conn, &anon_name, false).is_ok() {
                        context::switch_context(anon_name, None, true);
                        if context_changes_dir {
                            context::change_directory();
                        }
                    } else if db::get_challenge_from_name(&conn, anon_name.to_string()).is_ok() {
                        // get ctf name from challenge
                        let ctf_name =
                            db::get_ctf_name_from_challenge(&conn, anon_name.to_string()).unwrap();

                        context::switch_context(&ctf_name, Some(anon_name), true);
                        if context_changes_dir {
                            context::change_directory();
                        }
                        UndoAction::new_context_switch(&ctf_name, Some(&anon_name)).log_action();
                    } else {
                        println!(
                            "No ctf or challenge found with name {}",
                            anon_name
                        );
                    }
                }
                4 => {
                    // set context
                    let ctf_name = args[2].validate();
                    let chall_name = args[3].validate();
                    if let Ok(true) = is_ctf_archived(&db::get_conn(), &ctf_name) {
                        println!("{}Cannot switch context to [archived] {}", "✗".bright_red().bold(), ctf_name);
                        std::process::exit(1);
                    }
                    context::switch_context(ctf_name, Some(chall_name), true);
                    UndoAction::new_context_switch(ctf_name, Some(chall_name)).log_action();
                }
                _ => {
                    println!("{}Invalid number of arguments", "✗".bright_red().bold());
                    println!("Usage: tobi context - show current context");
                    println!("       tobi context <ctf> <challenge> - set current context");
                }
            }
        }
        "archive" => {
            match args.len() {
                2 => {
                    // archive current ctf context
                    let (ctf, _) = context::get_context();
                    if let None = ctf {
                        println!("{}No CTF found in context", "✗".bright_red().bold());
                        std::process::exit(1);
                    }
                    let ctf = ctf.unwrap();
                    let conn = db::get_conn();
                    if let Ok(true) = is_ctf_archived(&conn, &ctf.metadata.name) {
                        println!("CTF already archived");
                        std::process::exit(1);
                    }
                    ctf.archive();
                    context::save_context(None, None); // TODO figure bugs here
                    println!("^CHANGE_DIR^{}^CHANGE_DIR^", SETTINGS.lock().unwrap().workdir);
                    println!("Archived {}\nSwitching to CTFs path", ctf.metadata.name);
                }
                3 => {
                    let ctf_name = args[2].validate();
                    let conn = db::get_conn();
                    match is_ctf_archived(&conn, &ctf_name) {
                        Ok(a) => {
                            if a == true {
                                println!("{}CTF already archived", "✗".bright_red().bold());
                                std::process::exit(1);
                            }
                            let ctf = db::get_ctf_from_name(&conn, &ctf_name, false).unwrap();
                            // if pwd is the ctf directory, change to the parent directory
                            let workdir = SETTINGS.lock().unwrap().workdir.clone();
                            let pwd = std::env::current_dir().unwrap();
                            if pwd.starts_with(std::path::PathBuf::from(format!("{}/{}", workdir, ctf_name))) {
                                println!("^CHANGE_DIR^{}^CHANGE_DIR^", SETTINGS.lock().unwrap().workdir);
                            }
                            ctf.archive();
                            println!("Archived {}\nSwitching to CTFs path", ctf_name);
                        }
                        Err(_) => {
                            println!("{}CTF not found", "✗".bright_red().bold());
                            std::process::exit(1);
                        }
                    }
                }
                _ => {
                    println!("{}Invalid number of arguments", "✗".bright_red().bold());
                    println!("Usage: tobi archive - archive current ctf workspace");
                    println!("       tobi archive <ctf> - archive ctf workspace");
                }
            }
        }
        "unarchive" => {
            match args.len() {
                3 => {
                    let ctf_name = args[2].validate();
                    let conn = db::get_conn();
                    match is_ctf_archived(&conn, &ctf_name) {
                        Ok(a) => {
                            if a {
                                let ctf = db::get_ctf_from_name(&conn, &ctf_name, true).unwrap();
                                ctf.unarchive();
                                println!("Unarchived {}", ctf_name);
                            } else {
                                println!("{}CTF not archived", "✗".bright_red().bold());
                                std::process::exit(1);
                            }
                        }
                        Err(_) => {
                            println!("{}CTF not found", "✗".bright_red().bold());
                            std::process::exit(1);
                        }
                    }
                }
                _ => {
                    println!("{}Invalid number of arguments", "✗".bright_red().bold());
                    println!("       tobi unarchive <ctf> - unarchive ctf workspace");
                }
            }
        }
        "move" => {
            println!("{}Not implemented yet", "✗".bright_red().bold());
            std::process::exit(1);
            // if args.len() != 4 {
            //     println!("{}Invalid number of arguments", "✗".bright_red().bold());
            //     println!("Usage: tobi move <challenge> <ctf>");
            //     std::process::exit(1);
            // }
            // let new_ctf_name = args[3].validate();
            // if let Ok(true) = is_ctf_archived(&db::get_conn(), &new_ctf_name) {
            //     println!("{}Cannot move challenge to [archived] {}", "✗".bright_red().bold(), new_ctf_name);
            //     std::process::exit(1);
            // } else if db::get_ctf_from_name(&db::get_conn(), &new_ctf_name, false).is_err() {
            //     println!("{}CTF not found", "✗".bright_red().bold());
            //     std::process::exit(1);
            // }

            // let (ctf, chall) = context::get_context();
            // if let None = ctf {
            //     println!("{}No CTF found in context", "✗".bright_red().bold());
            //     std::process::exit(1);
            // }
            // if let None = chall {
            //     println!("{}No challenge found in context", "✗".bright_red().bold());
            //     std::process::exit(1);
            // }
            // let ctf = ctf.unwrap();
            // let chall = chall.unwrap();
            // let conn = db::get_conn();
            // db::move_challenge(&conn, &ctf.metadata.name, &chall.name, &new_ctf_name);
        }
        "undo" => {
            // undo the last action
            undo();
        }
        "settings" => {
            settings::show_settings_menu().unwrap();
        }
        "_autocomplete" => {
            // autocomplete for shell
            let prog_args = args[2..].join(" ");
            let split_args = prog_args
                .split_whitespace()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            autocompletions::print_completion(split_args);
        }
        _ => {
            println!("Invalid action");
        }
    }
}
