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
    /// An error that occures while parsing a semver version string.
    #[error("An error occurred during version parsing")]
    SemverParseError(#[from] semver::Error),
    /// An error that occurred during the toml parsing.
    #[error("a parser error occurred")]
    ParseError(#[from] TomlError),
    /// An error that gets emitted if the `package.version` field has not the
    /// right type (String).
    #[error("the field {field:?} is not of type {ty:?}")]
    InvalidFieldType { field: String, ty: String },
}

/// An enum defining what types of increments can be done to a semver version.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemVer {
    /// A major increment.
    Major,
    /// A minor increment.
    Minor,
    /// A patch increment.
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
pub fn get_version(path: impl AsRef<Path>) -> Result<Version, Error> {
    let cargo_toml_content = fs::read_to_string(path.as_ref())?;
    let doc = cargo_toml_content.parse::<Document>()?;
    let item: &Item = &doc["package"]["version"];

    // This should be the case for valid Cargo.toml files.
    if let Some(s) = item.as_str() {
        Ok(Version::parse(s)?)
    } else {
        Err(Error::InvalidFieldType {
            field: "version".to_string(),
            ty: "string".to_string(),
        })
    }
}

/// Sets the version inside a `Cargo.toml` file.
///
/// # Arguments
///
/// - `path`: The path to the `Cargo.toml` file.
/// - `version`: The version to write into the file. Note that no checks are
///   done to see whether the value contains a valid semver version.
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
pub fn bump_version(path: impl AsRef<Path>, r#type: SemVer) -> Result<Version, Error> {
    let mut version = get_version(path.as_ref())?;
    match r#type {
        SemVer::Major => version.increment_major(),
        SemVer::Minor => version.increment_minor(),
        SemVer::Patch => version.increment_patch(),
    }

    set_version(path, &version.to_string())?;
    Ok(version)
}

trait SemVerExt {
    fn increment_major(&mut self);
    fn increment_minor(&mut self);
    fn increment_patch(&mut self);
}

impl SemVerExt for Version {
    fn increment_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
    }

    fn increment_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
    }

    fn increment_patch(&mut self) {
        self.patch += 1;
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;
    use super::SemVerExt;

    use rstest::{fixture, rstest};

    #[fixture]
    pub fn fixture() -> Version {
        Version::new(1, 2, 3)
    }

    #[rstest]
    fn bump_patch(fixture: Version) {
        let mut fixture = fixture;
        fixture.increment_patch();
        assert_eq!(Version::new(1, 2, 4), fixture)
    }

    #[rstest]
    fn bump_minor(fixture: Version) {
        let mut fixture = fixture;
        fixture.increment_minor();
        assert_eq!(Version::new(1, 3, 0), fixture)
    }

    #[rstest]
    fn bump_major(fixture: Version) {
        let mut fixture = fixture;
        fixture.increment_major();
        assert_eq!(Version::new(2, 0, 0), fixture)
    }
}
