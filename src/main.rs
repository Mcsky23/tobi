use tobi::db;
use tobi::commands;
use tobi::settings;

fn main() {
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
            println!("Settings file not found. Opening settings menu...");
            // sleep to let the user read the message
            std::thread::sleep(std::time::Duration::from_secs(3));
            settings::show_settings_menu().unwrap();
            std::process::exit(1);
        }
    }

    let args = std::env::args().collect::<Vec<String>>();
    commands::do_action(args);
}
