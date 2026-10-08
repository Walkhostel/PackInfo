# Packinfo

## About

Packinfo can show info about all installed pacman packages in the system with their name, description, dependencies, and what requires them, like this:

<img width="982" height="101" alt="screenshot_2026-10-08_21-46-35" src="https://github.com/user-attachments/assets/0c73801c-6bf2-4833-bcf3-438625eabfed" />

## Installation

### From Cargo(rust)

```bash
cargo install packinfo
```
P.S. if after installation you get "command not found"-add `~/.cargo/bin` to PATH

## How to use

- `packinfo` for name and required by
- `packinfo [package name]` for info about a specific package
- `packinfo -D` for name, required by and depends
- `packinfo -d` for name, required by and description
- `packinfo -Dd` or `packinfo -dD` for name, required by, description and depends
- `packinfo --json` json output
