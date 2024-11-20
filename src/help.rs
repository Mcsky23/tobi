use crate::settings;
use fs_extra::dir::get_size;
use humansize::{format_size, DECIMAL};

pub fn print_help() {
    println!(r#"tobi v1.0.0

Usage:
    tobi [options]

Options:
    help                        Print this help message

    ctf                         change dir to the current CTF workspace
     |_ <ctf_name/chall_name>   change dir to the specified CTF or challenge
     |_ <ctf_name> <chall_name> change dir to the specified challenge

    new
     |_ ctf <ctf_name>          create a new CTF workspace and change dir to it
     |_ <category> <chall_name> create a new challenge and change dir to it

    edit
     |_ <category> <chall_name> edit the current chall's category and name
     |_ ctf <ctf_name>          edit the current CTF's name

    rm
     |_ <ctf_name>              remove the specified CTF
     |_ <ctf_name> <chall_name> remove the specified challenge

    context                     show the current context
     |_ <ctf_name/chall_name>   switch current context to the specified CTf or challenge
     |_ <ctf_name> <chall_name> switch current context to the specified challenge

    list                        list all challenges for the current CTF
     |_ all                     list all CTFs and challenges
     |_ ctf                     list all CTFs
     |_ <ctf_name>              list all challenges in the specified CTF
     |_ flags                   list all flags for the current CTF
     |_ archived                list all archived CTFs
     
    solve <flag>                submit a flag for the current challenge
    unsolve                     remove the flag for the current challenge

    undo                        undo the last action

    archive/unarchive           archive/unarchive the current CTF
     |_ <ctf_name>              archive/unarchive the specified CTF

    settings                    open the settings TUI menu
    "#);
}

pub fn print_info() {
    let ctf_size = get_size(settings::SETTINGS.lock().unwrap().workdir.clone()).unwrap();
    println!(r#"tobi v1.0.0
    Total size of CTF dir: {}"#, format_size(ctf_size, DECIMAL).to_string());
}