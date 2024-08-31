use crate::ctf;
use crate::settings;

pub fn do_action(args: Vec<String>) {
    let action = args[1].as_str();
    match action {
        "qnew" => {
            let what = args[2].as_str();
            match what {
                "ctf" => {
                    let name = &args[3];
                    ctf::quick_new(name.to_string());
                }
                _ => {
                    println!("Invalid action");
                }
            }

        }
        _ => {
            println!("Invalid action");
        }
    }
}