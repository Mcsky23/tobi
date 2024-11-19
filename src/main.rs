use tobi::db;
use tobi::commands;
use tobi::settings;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match settings::load_settings_from_file() {
        Ok(_) => { 
            match db::init_db() {
                Ok(_) => {}
                Err(_) => {
                    println!("Error loading settings from file. Run tobi settings and edit paths");
                    std::process::exit(1);
                }
            }
        },
        Err(_) => {
            if args.len() == 2 && args[1] == "settings"  {
                settings::show_settings_menu().unwrap();
                std::process::exit(0);
            } else {
                println!("tobi is not configured. Please run `tobi settings` to setup CTFs dir, db and context file paths.");
                std::process::exit(1);
            }
        }
    }
    commands::do_action(args);
}
