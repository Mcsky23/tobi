# TOBI

**Warning**: This is a work in progress and is **not** ready for use. Stuff does not work as intended and the code is not yet stable.

Tobi is a CLI tool designed to help with organizing and managing CTF challenge workspaces.

Command map:
- `tobi new`
    - `ctf <ctf_name>` - creates a new CTF workspace and switches CTF context to it
    - `<chall_category> <chall_name>` - creates a new challenge workspace and switches context to it
    - `flag <flag>` - solves the current challenge and saves the flag(use - as the flag to mark it as unsolved)

- `tobi list` - brief overview of all workspaces
    - `ctf` - lists all the CTF workspaces
    - `challs` - lists all the challenges in the current CTF workspace
    - `flags` - lists all the flags in the current challenge workspace

- `tobi context` - prints the current context
    - `<ctf_name>` - switches the current context to the specified CTF workspace
    - `<ctf_name> <chall_name>` - switches the current context to the specified challenge workspace

- `tobi switch <chall or CTF>` - switches the current context to the specified challenge or CTF workspace


### TODO:
- [ ] Implement directory switching
- [ ] Challenge name parsing
- [ ] Integrate ctf id into ctf struct
- [ ] Make context command accept a single parameter and determine if it is a ctf or chall(return option if multiple challs with same name)