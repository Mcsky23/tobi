use crate::ctf;
use crate::db;
use crate::context;

pub fn do_action(args: Vec<String>) {
    let action = args[1].as_str();
    match action {
        "new" => {
            let what = args[2].as_str();
            match what {
                "ctf" => {
                    if args.len() != 4 {
                        println!("Invalid number of arguments");
                        println!("Usage: tobi new ctf <name> - create a new ctf");
                    }
                    let name = &args[3];
                    ctf::quick_new(name.to_string());
                },
                chall_type => {
                    if args.len() != 4 {
                        println!("Invalid number of arguments");
                        println!("Usage: tobi new <type> <name> - create a new challenge");
                    }
                    let name = &args[3];
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
                    let ctf_name = &args[2];
                    context::switch_context(ctf_name, None);
                },
                4 => {
                    // set context
                    let ctf_name = &args[2];
                    let chall_name = &args[3];
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