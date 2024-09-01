use crate::ctf;
use crate::db;
use crate::context;

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
                    context::change_directory();
                    context::show_context();
                },
                _ => {
                    println!("Invalid number of arguments");
                    println!("Usage: tobi ctf - change to CTF directory");
                }
            }
        }
        "new" => {
            let what = args[2].validate().as_str();
            match what {
                "ctf" => {
                    if args.len() != 4 {
                        println!("Invalid number of arguments");
                        println!("Usage: tobi new ctf <name> - create a new ctf");
                    }
                    let name = args[3].validate();
                    ctf::quick_new(name.to_string());
                },
                chall_type => {
                    if args.len() != 4 {
                        println!("Invalid number of arguments");
                        println!("Usage: tobi new <type> <name> - create a new challenge");
                    }
                    let name = args[3].validate();
                    ctf::new_challenge(name.to_string(), chall_type.to_string());
                }
            }

        },
        "list" => {
            // TODO: list ctfs individually
            // For now, just list all ctfs
            let ctfs = db::get_all_ctfs().unwrap();
            if ctfs.len() == 0 {
                println!("No ctfs found");
            }
            for ctf in ctfs {
                println!("{}", ctf.metadata.name);
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
                    // figure if this is a ctf or a challenge by searching through db
                    if db::get_ctf_from_name(anon_name.to_string()).is_ok() {
                        context::switch_context(anon_name, None);
                    } else if db::get_challenge_from_name(anon_name.to_string()).is_ok() {
                        // get ctf name from challenge
                        let ctf_name = db::get_ctf_name_from_challenge(anon_name.to_string()).unwrap();

                        context::switch_context(&ctf_name, Some(anon_name));
                    } else {
                        println!("No ctf or challenge found with name {}", anon_name);
                    }
                },
                4 => {
                    // set context
                    let ctf_name = args[2].validate();
                    let chall_name = args[3].validate();
                    context::switch_context(ctf_name, Some(chall_name));
                },
                _ => {
                    println!("Invalid number of arguments");
                    println!("Usage: tobi context - show current context");
                    println!("       tobi context <ctf> <challenge> - set current context");
                }
            }
        },
        _ => {
            println!("Invalid action");
        }
    }
}