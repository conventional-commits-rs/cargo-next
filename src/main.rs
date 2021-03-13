use cargo_next::{bump_version, get_version, set_version, SemVer};
use clap::{AppSettings, Clap};
use std::{env::current_dir, io, process::exit};

#[derive(Clap, Debug)]
#[clap(
    author,
    bin_name("cargo-next"),
    setting(AppSettings::ColoredHelp),
    version
)]
enum Cli {
    #[clap(
        name = "next",
        setting(AppSettings::DeriveDisplayOrder),
        setting(AppSettings::UnifiedHelpMessage)
    )]
    Next(Args),
}

#[derive(Clap, Debug)]
struct Args {
    /// Returns the current version of a crate.
    #[clap(long)]
    pub get: bool,

    /// Increment the crate's major version.
    #[clap(long)]
    pub major: bool,

    /// Increment the crate's minor version.
    #[clap(long)]
    pub minor: bool,

    /// Increment the crate's patch version.
    #[clap(long)]
    pub patch: bool,

    /// The version to set the crate to.
    pub version: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut cli = match cli {
        Cli::Next(args) => args,
    };

    // If no flag has been specified and no version, read from stdin.
    if !cli.major && !cli.minor && !cli.patch && !cli.get && cli.version.is_none() {
        let mut piped = String::new();
        io::stdin().read_line(&mut piped)?;
        let piped_trim = piped.trim();
        if !piped_trim.is_empty() {
            cli.version = Some(piped_trim.to_string());
        }
    }

    // Check if the current directory is actually a cargo project.
    let cargo_project_dir_path = current_dir()?;
    let cargo_toml_file_path = cargo_project_dir_path.join("Cargo.toml");
    if !cargo_toml_file_path.exists() {
        eprintln!("Not inside a cargo project folder!");
        exit(1);
    }

    if cli.get {
        println!("{}", get_version(&cargo_toml_file_path)?);
    } else if cli.major {
        bump_version(&cargo_toml_file_path, SemVer::Major)?;
    } else if cli.minor {
        bump_version(&cargo_toml_file_path, SemVer::Minor)?;
    } else if cli.patch {
        bump_version(&cargo_toml_file_path, SemVer::Patch)?;
    } else {
        // Safety: Either `version` contains a String supplied from the user or the CLI
        // waits until it can read from stdin, in which case a version gets set as well.
        set_version(&cargo_toml_file_path, cli.version.unwrap())?;
    }

    Ok(())
}
