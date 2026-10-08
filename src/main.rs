use comfy_table::{Table, presets::UTF8_FULL};
use anyhow::Result;
use serde::Serialize;
use std::process::Command;
use pager::Pager;

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
        .map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty())
        .collect()
}

fn render_deps(title: &str, deps: &[String]) {
    println!("\n{title} ({}):", deps.len());
    if deps.is_empty() {
        println!("  empty");
        return;
    }
    let mut t = Table::new();
    t.load_preset(UTF8_FULL);
    t.set_header(["dep"]);
    for d in deps {
        t.add_row([d]);
    }
    println!("{t}");
}

fn main() -> Result<()> {
    let pkgs = load()?;
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    let show_desc = args.iter().any(|a| a.contains('d') && a.starts_with('-'));
    let show_deps = args.iter().any(|a| a.contains('D') && a.starts_with('-'));
    let name_filter = args.iter().find(|a| !a.starts_with('-'));

    let filtered: Vec<&Package> = match name_filter {
        Some(name) => pkgs.iter().filter(|p| p.name == *name).collect(),
        None => pkgs.iter().collect(),
    };

    if filtered.is_empty() {
        if let Some(name) = name_filter {
            eprintln!("package {name} not found");
        }
        return Ok(());
    }

    if filtered.len() > 1 {
        Pager::with_pager("less -RS").setup();
    }

    let mut t = Table::new();
    t.load_preset(UTF8_FULL);

    let mut headers = vec!["name", "required by"];
    if show_desc { headers.push("desc"); }
    if show_deps { headers.push("depends"); }
    t.set_header(&headers);

    t.column_mut(0).unwrap()
        .set_constraint(comfy_table::ColumnConstraint::UpperBoundary(comfy_table::Width::Fixed(25)));
    t.column_mut(1).unwrap()
        .set_constraint(comfy_table::ColumnConstraint::UpperBoundary(comfy_table::Width::Fixed(30)));

    for p in filtered {
        let mut row = vec![p.name.clone(), p.required_by.join(", ")];
        if show_desc { row.push(p.description.clone()); }
        if show_deps { row.push(p.depends.join(", ")); }
        t.add_row(row);
    }

    println!("{t}");
    Ok(())
}
