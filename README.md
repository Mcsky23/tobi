# TOBI

Tobi is a CLI tool designed to help with organizing and managing CTF challenge workspaces.

Command map:
- `tobi qnew` (will change to `tobi new` in the future) - creates a new challenge workspace
    - `ctf <ctf_name>` - creates a new CTF workspace and switches CTF context to it
    - `<chall_category> <chall_name>` - creates a new challenge workspace and switches context to it
    - `flag <flag>` - solves the current challenge and saves the flag(use - as the flag to mark it as unsolved)

- `tobi list` - gives a brief overview of the current workspace
    - `ctfs` - lists all the CTF workspaces
    - `challs` - lists all the challenges in the current CTF workspace
    - `flags` - lists all the flags in the current challenge workspace

- `tobi switch <chall or CTF>` - switches the current context to the specified challenge or CTF workspace
