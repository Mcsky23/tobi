pub mod ctf;
pub mod commands;
pub mod settings;
pub mod db;

fn main() {
    db::init();
    
    let args = std::env::args().collect::<Vec<String>>();
    commands::do_action(args);
}
