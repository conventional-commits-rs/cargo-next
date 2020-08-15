use pico_args::Arguments;
use std::env::{args_os, current_dir};
use std::fs;
use std::io;
use std::process::exit;
use toml_edit::{value, Document};
use cargo_next::set_version;

const HELP: &str = "USAGE: cargo next <VERSION>";

struct Args {
    help: bool,
    next_version: Vec<String>,
    version: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargo subcommands need special handling.
    let args_to_use = args_os().skip(2).collect();
    let mut args = Arguments::from_vec(args_to_use);

    let mut args = Args {
        help: args.contains(["-h", "--help"]),
        version: args.contains(["-V", "--version"]),
        next_version: args.free()?,
    };

    // Check standard-in to allow for piping.
    let mut piped = String::new();
    io::stdin().read_line(&mut piped)?;
    let piped_trim = piped.trim();
    if !piped_trim.is_empty() {
        args.next_version.push(piped_trim.to_string());
    }

    if args.help {
        println!("{}", HELP);
    } else if args.version {
        println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    } else if args.next_version.len() != 1 {
        eprintln!(
            "Only one version can be specified! Passed: {:?}",
            args.next_version
        );
        eprintln!("{}", HELP);
        exit(1);
    } else {
        // Check if the current directory is actually a cargo project.
        let cargo_project_dir_path = current_dir()?;
        let cargo_toml_file_path = cargo_project_dir_path.join("Cargo.toml");
        if !cargo_toml_file_path.exists() {
            eprintln!("Not inside a cargo project folder!");
            exit(1);
        }

        set_version(&cargo_toml_file_path, &args.next_version[0]);
    }

    Ok(())
}
