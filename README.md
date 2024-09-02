# TOBI

**Warning**: This is a work in progress and is **not** ready for use. Stuff does not work as intended and the code is not yet stable.

Tobi is a CLI tool designed to help with organizing and managing CTF challenge workspaces.

Command map:
- `tobi ctf` - change dir to the current CTF workspace(if no chall is in scope, it will change to the CTF base dir)
    - `tobi ctf <ctf_name/chall_name>` - change dir to the specified CTF workspace
    - `tobi ctf <ctf_name> <chall_name>` - change dir to the specified challenge workspace

- `tobi new`
    - `ctf <ctf_name>` - creates a new CTF workspace and switches CTF context to it
    - `<chall_category> <chall_name>` - creates a new challenge workspace and switches context to it
    - `flag <flag>` - solves the current challenge and saves the flag(use - as the flag to mark it as unsolved)

- `tobi list` - list all ctfs
    - `all` - lists all ctfs and all challenges
    - `<ctf_name>` - lists all challenges in the specified CTF
    - `flags` - lists all the flags in the current challenge workspace

- `tobi context` - prints the current context and a couple stats(Notice similarity to `tobi ctf`. The latter is for changing directories, the former is for changing the context)
    - `<ctf_name>` - switches the current context to the specified CTF workspace
    - `<ctf_name> <chall_name>` - switches the current context to the specified challenge workspace

## Installation
Because changing the shell's directory from a running child process is not possible, `tobi` uses a wrapper bash script. This declares a helper function that ingests `tobi-cli`(the actual binary) output and changes the directory if needed. `tobi` wrapper is sourced in the shell's rc file.

To install `tobi` clone the repo and simply run the install script with the needed arguments:
```bash
./install.sh --install-dir=<desired_install_dir> --rc-file=<shell rc file>

# Example:
./install.sh --install-dir=/opt/scripts --rc-file=$(realpath ~/.zshrc)
```

### TODO:
- [x] Implement directory switching
- [x] Challenge name parsing
- [x] Make context command accept a single parameter and determine if it is a ctf or chall(return option if multiple challs with same name)
- [x] Add checkboxes to `tobi list` output if the challenge is solved
- [ ] Implement undo command
- [x] Implement flag/solve command
- [ ] Backup db to some cloud service(will look into this later)
- [ ] Implement TUI settings menu
- [ ] Create custom scripts for a workspace or for all workspaces
- [ ] Remote pwn environment integration
- [ ] Beautify prints
- [ ] Make `tobi` command customizable
- [ ] Add setting to automatically switch directory when switching context