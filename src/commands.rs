use crate::autocompletions;
use crate::ctf;
use crate::db;
use crate::context;
use crate::undo::{UndoAction, undo};
use crate::settings;

trait ArgName<T> {
    fn validate(&self) -> &T;
}

impl ArgName<String> for String {
    fn validate(&self) -> &String {
        // check that characters are alphanumeric or underscore
        if self.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return &self;
        }
        println!("Invalid name in argument: {}", self);
        std::process::exit(1);
    }
}

pub fn do_action(args: Vec<String>) {
    if args.len() == 1 {
        context::show_context();
    }
    let action = args[1].validate().as_str();
    match action {
        "ctf" => {
            match args.len() {
                2 => {
                    UndoAction::new_dir_change().log_action();
                    context::change_directory();
                    context::show_context();
                },
                3 => {
                    // change directory to specified ctf but don't change the context
                    // check if ctf exists
                    let ctf_name = args[2].validate();
                    let conn = db::get_conn();
                    match db::get_ctf_from_name(&conn, ctf_name.to_string()) {
                        Ok(ctf) => {
                            UndoAction::new_dir_change().log_action();
                            println!("CHANGE_DIR: {}", ctf.file_path);
                        },
                        Err(_) => {
                            println!("CTF not found");
                        }
                    }
                    
                },
                4 => {
                    // change directory to specified ctf and challenge
                    let ctf_name = args[2].validate();
                    let chall_name = args[3].validate();
                    let conn = db::get_conn();

                    match db::get_ctf_from_name(&conn, ctf_name.to_string()) {
                        Ok(ctf) => {
                            match db::get_challenge_from_name(&conn, chall_name.to_string()) {
                                Ok(chall) => {
                                    UndoAction::new_dir_change().log_action();
                                    println!("CHANGE_DIR: {}/{}", ctf.file_path, chall.category);
                                },
                                Err(_) => {
                                    println!("Challenge not found");
                                }
                            }
                        },
                        Err(_) => {
                            println!("CTF not found");
                        }
                    }
                }
                _ => {
                    println!("Invalid number of arguments");
                    println!("Usage: tobi ctf - change to CTF directory");
                }
            }
        },
        "new" => {
            let what = args[2].validate().as_str();
            match what {
                "ctf" => {
                    if args.len() != 4 {
                        println!("Invalid number of arguments");
                        println!("Usage: tobi new ctf <name> - create a new ctf");
                    }
                    let name = args[3].validate();
                    UndoAction::new_ctf_create(name).log_action();
                    ctf::quick_new(name.to_string());
                },
                chall_type => {
                    if args.len() != 4 {
                        println!("Invalid number of arguments");
                        println!("Usage: tobi new <type> <name> - create a new challenge");
                    }
                    let name = args[3].validate();
                    ctf::new_challenge(name.to_string(), chall_type.to_string());
                    UndoAction::new_chall_create(&name).log_action(); // no need to error check here, if there is no ctf in scope program exits anyways
                }
            }
            
        },
        "list" => {
            // For now, just list all ctfs
            match args.len() {
                2 => { // list all challenges in current ctf
                    let (ctf, _) = context::get_context();
                    if let None = ctf {
                        println!("No CTF found in context");
                        std::process::exit(1);
                    }
                    let ctf = ctf.unwrap();
                    ctf.print_challs();
                },
                3 => { 
                    match args[2].as_str() {
                        "all" => { // list all chalenges in all ctfs
                            let ctfs = db::get_all_ctfs(&db::get_conn()).unwrap();
                            if ctfs.len() == 0 {
                                println!("No ctfs found");
                            }
                            for ctf in ctfs {
                                ctf.print_challs();
                                println!();
                            }
                            return;
                        },
                        "ctfs" => { // list all ctf names
                            let ctfs = db::get_all_ctfs(&db::get_conn()).unwrap();
                            if ctfs.len() == 0 {
                                println!("No ctfs found");
                            }
                            for ctf in ctfs {
                                println!("{}", ctf.metadata.name);
                            }
                        },
                        "flags" => { // list all challenges with flags
                            todo!();
                        }
                        _ => {
                            let ctf_name = args[2].validate();
                            let conn = db::get_conn();
                            let ctf = db::get_ctf_from_name(&conn, ctf_name.to_string()).unwrap_or_else(|_| {
                                println!("CTF not found");
                                std::process::exit(1);
                            });
                            
                            ctf.print_challs();
                        }
                    }
                },
                _ => {
                    println!("Invalid number of arguments");
                    println!("Usage: tobi list - list all ctfs");
                    println!("       tobi list <ctf> - list all challenges in ctf");
                }
            }
            
        },
        "solve" => {
            // solve the current challenge
            let (ctf, challenge) = context::get_context();
            if let None = ctf {
                println!("No CTF found in context");
                std::process::exit(1);
            }
            let ctf = ctf.unwrap();
            if args.len() != 3 {
                println!("Invalid number of arguments");
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
                    println!("Solved {} -> {} [{}]: {}", &ctf.metadata.name, &challenge.name, &challenge.category, &challenge.flag);
                },
                None => {
                    println!("You are currently not working on a challenge");
                    std::process::exit(1);
                }
            }
        },
        "unsolve" => {
            // remove the flag from the current challenge
            // solve the current challenge
            let (ctf, challenge) = context::get_context();
            if let None = ctf {
                println!("No CTF found in context");
                std::process::exit(1);
            }
            let ctf = ctf.unwrap();
            if args.len() != 2 {
                println!("Invalid number of arguments");
                println!("Usage: tobi solve <flag>");
                std::process::exit(1);
            }
            match challenge {
                Some(mut challenge) => {
                    // solve challenge
                    UndoAction::new_chall_unsolve(&ctf.metadata.name, &challenge.name, &challenge.flag).log_action();
                    challenge.flag = "".to_string();
                    challenge.save_to_db(&ctf.metadata.name);
                    println!("Unsolved {} -> {} [{}]", &ctf.metadata.name, &challenge.name, &challenge.category);
                },
                None => {
                    println!("You are currently not working on a challenge");
                    std::process::exit(1);
                }
            }
        },
        "context" => {
            match args.len() {
                2 => {
                    // show context
                    context::show_context();
                },
                3 => {
                    // set context
                    let anon_name = args[2].validate();
                    let conn = db::get_conn();
                    // figure if this is a ctf or a challenge by searching through db
                    if db::get_ctf_from_name(&conn, anon_name.to_string()).is_ok() {
                        context::switch_context( anon_name, None);
                    } else if db::get_challenge_from_name(&conn, anon_name.to_string()).is_ok() {
                        // get ctf name from challenge
                        let ctf_name = db::get_ctf_name_from_challenge(&conn, anon_name.to_string()).unwrap();
                        
                        context::switch_context(&ctf_name, Some(anon_name));
                        UndoAction::new_context_switch(&ctf_name, Some(&anon_name)).log_action();
                    } else {
                        println!("No ctf or challenge found with name {}", anon_name);
                    }
                },
                4 => {
                    // set context
                    let ctf_name = args[2].validate();
                    let chall_name = args[3].validate();
                    context::switch_context(ctf_name, Some(chall_name));
                    UndoAction::new_context_switch(ctf_name, Some(chall_name)).log_action();
                },
                _ => {
                    println!("Invalid number of arguments");
                    println!("Usage: tobi context - show current context");
                    println!("       tobi context <ctf> <challenge> - set current context");
                }
            }
        },
        "undo" => {
            // undo the last action
            undo();
        },
        "settings" => {
            settings::show_settings_menu().unwrap();
        },
        "_autocomplete" => {
            // autocomplete for shell
            let prog_args = args[2..].join(" ");
            let split_args = prog_args.split_whitespace().into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

            autocompletions::print_completion(split_args);
        }
        _ => {
            println!("Invalid action");
        }
    }
}