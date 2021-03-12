use semver::Version;
use std::{fs, io::Error as IoError, path::Path};
use thiserror::Error;
use toml_edit::{value, Document, Item, TomlError};

/// The error type of this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// An error that occurred during the read and write operation of the
    /// `Cargo.toml` file.
    #[error("an io error occurred")]
    IoError(#[from] IoError),
    #[error("An error occurred during version parsing")]
    SemverParseError(#[from] semver::SemVerError),
    /// An error that occurred during the toml parsing.
    #[error("a parser error occurred")]
    ParseError(#[from] TomlError),
    /// An error that gets emitted if the `package.version` field has not the
    /// right type (String).
    #[error("the field {field:?} is not of type {ty:?}")]
    InvalidFieldType { field: String, ty: String },
}

#[derive(Debug, PartialEq)]
pub enum SemVer {
    Major,
    Minor,
    Patch,
}

/// Returns the version inside a `Cargo.toml` file.
///
/// # Arguments
///
/// - `path`: The path to the `Cargo.toml` file.
///
/// # Returns
///
/// The version as a `String` if it could be successfully extracted, otherwise
/// an error.
pub fn get_version(path: impl AsRef<Path>) -> Result<String, Error> {
    let cargo_toml_content = fs::read_to_string(path.as_ref())?;
    let doc = cargo_toml_content.parse::<Document>()?;
    let item: &Item = &doc["package"]["version"];

    // This should be the case for valid Cargo.toml files.
    item.as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| Error::InvalidFieldType {
            field: "version".to_string(),
            ty: "string".to_string(),
        })
}

/// Sets the version inside a `Cargo.toml` file.
///
/// # Arguments
///
/// - `path`: The path to the `Cargo.toml` file.
///
/// # Returns
///
/// An error if something went wrong during IO operations or parsing.
pub fn set_version(path: impl AsRef<Path>, version: impl AsRef<str>) -> Result<(), Error> {
    let cargo_toml_content = fs::read_to_string(path.as_ref())?;
    let mut doc = cargo_toml_content.parse::<Document>()?;

    doc["package"]["version"] = value(version.as_ref());
    fs::write(path.as_ref(), doc.to_string())?;

    Ok(())
}

/// Bumps the version inside a `Cargo.toml` file according to semver specs.
///
/// # Arguments
///
/// - `path`: The path to the `Cargo.toml` file.
/// - `type`: The type of bump. Either patch, minor or major.
///
/// # Returns
///
/// The new version or an error if something went wrong during IO operations.
pub fn bump_version(path: impl AsRef<Path>, r#type: SemVer) -> Result<String, Error> {
    let version = get_version(path)?;
    let mut version: Version = Version::parse(&version)?;
    match r#type {
        SemVer::Major => version.increment_major(),
        SemVer::Minor => version.increment_minor(),
        SemVer::Patch => version.increment_patch(),
    }
    Ok(version.to_string())
}
