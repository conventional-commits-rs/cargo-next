use std::fs;
use std::io::Error as IoError;

use toml_edit::{value, Document};
use std::path::Path;
use thiserror::Error;
use toml_edit::{TomlError, Item};

/// The error type of this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// An error that occurred during the read and write operation of the `Cargo.toml` file.
    #[error("an io error occurred")]
    IoError(#[from] IoError),
    /// An error that occurred during the toml parsing.
    #[error("a parser error occurred")]
    ParseError(#[from] TomlError),
    /// An error that gets emitted if the `package.version` field has not the right type (String).
    #[error("the field {field:?} is not of type {ty:?}")]
    InvalidFieldType {
        field: String,
        ty: String,
    },
}

/// Returns the version inside a `Cargo.toml` file.
///
/// # Arguments
///
/// - `path`: The path to the `Cargo.toml` file.
///
/// # Returns
///
/// The version as a `String` if it could be successfully extracted, otherwise an error.
pub fn get_version(path: impl AsRef<Path>) -> Result<String, Error> {
    let cargo_toml_content = fs::read_to_string(path.as_ref())?;
    let doc = cargo_toml_content.parse::<Document>()?;
    let item: &Item = &doc["package"]["version"];

    // This should be the case for valid Cargo.toml files.
    item.as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| Error::InvalidFieldType {
            field: "version".to_string(),
            ty: "string".to_string()
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
    fs::write(cargo_toml_file.as_ref(), doc.to_string())?;

    Ok(())
}
