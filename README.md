# Packinfo

## About

Packinfo can show info about all installed pacman packages in the system with their name, description, dependencies, and what requires them, like this:

<img width="983" height="105" alt="image" src="https://github.com/user-attachments/assets/5ad7090d-c8b7-41cf-998c-8ec9aa40df6b" />

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
