# Packinfo

## About

Packinfo shows info about all installed pacman packages in the system with their name, description, dependencies, and what requires them, like this:

<img width="459" height="53" alt="screenshot_2026-10-05_21-12-09" src="https://github.com/user-attachments/assets/7500e824-5ed3-44a4-a2c8-70247c4e0eed" />

## Installation

### From Cargo (crates.io)

```bash
cargo install packinfo
```

### From source

```bash
git clone https://github.com/Walkhostel/packinfo
cd packinfo
cargo install --path .
```

## How to use

- `packinfo` for the full list
- `packinfo [package name]` for info about a specific package
