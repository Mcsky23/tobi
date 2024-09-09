pub fn print_help() {
    println!(r#"tobi v0.1.0

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

    context                     show the current context
     |_ <ctf_name/chall_name>   switch current context to the specified CTf or challenge
     |_ <ctf_name> <chall_name> switch current context to the specified challenge

    list                        list all challenges for the current CTF
     |_ all                     list all CTFs and challenges
     |_ ctf                     list all CTFs
     |_ <ctf_name>              list all challenges in the specified CTF
     |_ flags                   list all flags for the current CTF
     
    solve <flag>                submit a flag for the current challenge
    unsolve                     remove the flag for the current challenge

    undo                        undo the last action

    settings                    open the settings TUI menu
    "#);
}