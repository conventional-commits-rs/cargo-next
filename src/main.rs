use pico_args::Arguments;
use std::env::{args_os, current_dir};
use std::fs;
use std::process::exit;
use toml_edit::{value, Document};

const HELP: &str = "USAGE: cargo next <VERSION>";

struct Args {
    help: bool,
    next_version: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargo subcommands need special handling.
    let args_to_use = args_os().skip(2).collect();
    let mut args = Arguments::from_vec(args_to_use);

    let args = Args {
        help: args.contains(["-h", "--help"]),
        next_version: args.free()?,
    };
    if args.next_version.len() != 1 {
        eprintln!("{}", HELP);
        exit(1);
    } else if args.help {
        println!("{}", HELP);
    } else {
        // Check if the current directory is actually a cargo project.
        let cargo_project_dir_path = current_dir()?;
        let cargo_toml_file_path = cargo_project_dir_path.join("Cargo.toml");
        if !cargo_toml_file_path.exists() {
            eprintln!("Not inside a cargo project folder!");
            exit(1);
        }

        // Read and modify Cargo.toml.
        let cargo_toml_content = fs::read_to_string(&cargo_toml_file_path)?;
        let mut doc = cargo_toml_content.parse::<Document>()?;
        doc["package"]["version"] = value(args.next_version[0].as_ref());

        // Write file back.
        fs::write(cargo_toml_file_path, doc.to_string())?;
    }
    Ok(())
}
