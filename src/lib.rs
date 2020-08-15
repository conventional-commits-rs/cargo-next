use std::fs;
use std::io::Error as IoError;

use toml_edit::{value, Document};
use std::path::Path;
use thiserror::Error;
use toml_edit::{TomlError, Item};

#[derive(Debug, Error)]
pub enum Error {
    #[error("an io error occurred")]
    IoError(#[from] IoError),
    #[error("a parser error occurred")]
    ParseError(#[from] TomlError),
    #[error("the field {field:?} is not of type {ty:?}")]
    InvalidFieldType {
        field: String,
        ty: String,
    },
}

pub fn get_version(cargo_toml_file: impl AsRef<Path>) -> Result<String, Error> {
    let cargo_toml_content = fs::read_to_string(cargo_toml_file.as_ref())?;
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

pub fn set_version(cargo_toml_file: impl AsRef<Path>, version: impl AsRef<str>) -> Result<(), Error> {
    let cargo_toml_content = fs::read_to_string(cargo_toml_file.as_ref())?;
    let mut doc = cargo_toml_content.parse::<Document>()?;

    doc["package"]["version"] = value(version.as_ref());
    fs::write(cargo_toml_file.as_ref(), doc.to_string())?;

    Ok(())
}
