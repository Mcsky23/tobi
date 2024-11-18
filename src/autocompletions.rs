use crate::db;

pub fn print_completion(args: Vec<String>) {
    match args[0].parse::<i32>().unwrap() {
        1 => { // tobi commands
            println!("ctf");
            println!("new");
            println!("edit");
            println!("rm");
            println!("list");
            println!("context");
            println!("solve");
            println!("unsolve");
            println!("undo");
            println!("settings");
            println!("archive");
            println!("unarchive");
        },
        2 => {
            match args[1].as_str() {
                "ctf" | "context" | "list" | "rm" | "archive" => {
                    // print all ctf names
                    let conn = db::get_conn();
                    let ctfs = db::get_all_ctfs(&conn, false).unwrap();
                    for ctf in ctfs {
                        println!("{}", ctf.metadata.name);
                    }
                },
                "unarchive" => {
                    let conn = db::get_conn();
                    let ctfs = db::get_all_ctfs(&conn, true).unwrap();
                    for ctf in ctfs {
                        println!("{}", ctf.metadata.name);
                    }
                },
                "new" => {
                    println!("ctf");
                    // print all challenge type from ChallengeType enum
                    println!("web");
                    println!("pwn");
                    println!("crypto");
                    println!("misc");
                    println!("reversing");
                    println!("forensics");
                },
                _ => {}
            }
        },
        3 => {
            match args[1].as_str() {
                "ctf" | "context" | "rm" => {
                    // print all challenge names from ctf
                    let ctf_name = &args[2];
                    let conn = db::get_conn();
                    let ctf = db::get_ctf_from_name(&conn, &ctf_name, false).unwrap_or_else(|_| {
                        std::process::exit(1);
                    });
                    for chall in ctf.challenges {
                        println!("{}", chall.name);
                    }
                },
                _ => {}
            }
        }
        _ => {}
    }
}