use comfy_table::{Table, presets::UTF8_FULL};
use anyhow::Result;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct Package {
    name: String,
    description: String,
    depends: Vec<String>,
    required_by: Vec<String>,
}

fn load() -> Result<Vec<Package>> {
    let out = Command::new("expac")
        .args(["-Q", "%n\x1f%d\x1f%D\x1f%R"])
        .output()?;

    let text = String::from_utf8(out.stdout)?;
    let mut pkgs = Vec::new();

    for line in text.lines() {
        let f: Vec<&str> = line.split('\x1f').collect();
        if f.len() < 4 {
            continue;
        }
        pkgs.push(Package {
            name: f[0].into(),
            description: f[1].into(),
            depends: split_deps(f[2]),
            required_by: split_deps(f[3]),
        });
    }
    Ok(pkgs)
}

fn split_deps(s: &str) -> Vec<String> {
    s.split(", ")
        .map(|d| d.split(['>', '<', '=']).next().unwrap().trim().to_string())
        .filter(|d| !d.is_empty())
        .collect()
}

fn main() -> Result<()> {
    let pkgs = load()?;
    let arg = std::env::args().nth(1);
    match arg.as_deref() {
        None => {
            let mut t = Table::new();
            t.set_header(["name", "desc"]);           // вместо load_preset
            for p in &pkgs {
                t.add_row([&p.name, &p.description]);
            }
            println!("{t}");
        }
        Some("--json") => println!("{}", serde_json::to_string_pretty(&pkgs)?),
        Some(name) => {
            if let Some(p) = pkgs.iter().find(|p| p.name == name) {
                println!("name:     {}", p.name);
                println!("desc:     {}", p.description);
                println!("depends:  {}", p.depends.join(", "));
                println!("req by:   {}", p.required_by.join(", "));
            } else {
                eprintln!("пакет {name} не найден");
            }
        }
    }

    Ok(())
}
