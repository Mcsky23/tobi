# TOBI

Tobi is a CLI tool designed to help with organizing and managing CTF challenge workspaces.

Command map:
- `tobi` by itself will be aliased to `tobi ctf` (this can be changed in the settings menu)
- `tobi ctf` - change dir to the current CTF workspace(if no chall is in scope, it will change to the CTF base dir)
    - `tobi ctf <ctf_name/chall_name>` - change dir to the specified CTF workspace
    - `tobi ctf <ctf_name> <chall_name>` - change dir to the specified challenge workspace

        Note: `tobi ctf` will **NOT** change context, only the directory

- `tobi new`
    - `ctf <ctf_name>` - creates a new CTF workspace and switches CTF context to it
    - `<chall_category> <chall_name>` - creates a new challenge workspace and switches context to it

- `tobi edit <chall_category> <chall_name>` - edit the current challenge's category and name 
    - `ctf <ctf_name>` - edit the current CTF's name

- `tobi rm <ctf_name>` - removes the specified CTF workspace including all challenges
    - `tobi rm <ctf_name> <chall_name>` - removes the specified challenge workspace

- `tobi list` - list all challenges in the current context
    - `all` - lists all ctfs and all challenges
    - `ctf` - list all ctf names
    - `<ctf_name>` - lists all challenges in the specified CTF
    - `flags` - lists all the flags in the current challenge workspace
    - `archived` - lists all archived CTFs

- `tobi context` - prints the current context and a couple stats(Notice similarity to `tobi ctf`. The latter is for changing directories, the former is for changing the context)
    - `<ctf_name/chall_name>` - switches the current context to the specified CTF workspace
    - `<ctf_name> <chall_name>` - switches the current context to the specified challenge workspace

- `tobi solve <flag>` - marks the current challenge as solved and saves the flag. If you want to change the flag, you can simply run this command again with the new flag
- `tobi unsolve` - marks the current challenge as unsolved

- `tobi undo` - undoes the last action

    Note: An example use case for `tobi undo` is checking out some other CTF files. When it's time to go back to the context directory, instead of typing the challenge name, you can simply run this command and it will take you back.

- `tobi archive/unarchive` - archives/unarchives the current CTF workspace
    - `<ctf_name>` - archives/unarchive the specified CTF workspace

- `tobi settings` - opens TUI settings menu

## Quick setup

### Installation

To install `tobi`:
- make sure you have `cargo` installed
- clone this repo
- run the install script with the needed arguments:
```bash
./install.sh --install-dir=<desired_install_dir>

# Example:
./install.sh --install-dir=/opt/scripts
```

### Uninstall
```bash
./install.sh --uninstall
```

### Info

When installing, the script will:
- compile the binary using `cargo`
- copy the binary and the wrapper script to the specified directory
- add `source <install_dir>/tobirc.sh` to your shell's rc file

## Inner workings

`tobi` uses a sqlite database to store information about the CTFs and challenges. For proper functionality, it relies on 3 files that are created on your system:
- `DB_PATH/tobi.db` - the sqlite database
- `CONTEXT_PATH/.tobicntxt` - the file that remembers the current context so that you can easily switch back to it
- `/tmp/tobi` - a temporary file containing the last action so that you can undo it

Because changing the shell's directory from a running child process is not possible, `tobi` uses a wrapper bash script. This declares a helper function that ingests `tobi-cli`(the actual binary) output and changes the directory if needed. `tobi` wrapper is sourced in the shell's rc file.

Note: Tobi also supports `tab auto-completion`. The wrapper script contains the auto-completion function.

## Settings

Tobi settings(`tobi settings`) feature a TUI interface implemented using [ratatui](https://ratatui.rs). You can customize stuff like where tobi stores it's database or where CTF and challenge workspaces are created. It's also possible to customize the behavior of the `tobi` command.

<img src="./demo_img/main_menu1.png" width="100%">

### Planned updates
- [ ] Add challenge move functionality
- [ ] Backup db to some cloud service
- [ ] Extend undo functionality to more than one action
- [ ] Remote pwn environment integration
- [ ] Add setting to automatically switch directory when switching context