use tobi::db;
use tobi::commands;

fn main() {
    db::init();

    let args = std::env::args().collect::<Vec<String>>();
    commands::do_action(args);
}
