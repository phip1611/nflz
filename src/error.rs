//! Module for errors inside NFLZ library.

use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum NFLZError {
    FilenameMustIncludeExactlyOneNumberedGroup(String),
    ValueInNumberedGroupInvalid(String),
    CantReadDirectory(String, std::io::Error),
    ConflictingFiles(Vec<String>),
    RenameFailed(String, String, std::io::Error),
    AmbiguousSuffixes(HashSet<String>),
    AmbiguousPrefixes(HashSet<String>),
}

impl NFLZError {
    /// The filename that resulted in an error.
    pub fn filename(&self) -> Option<&str> {
        match self {
            NFLZError::FilenameMustIncludeExactlyOneNumberedGroup(fln) => {
                Option::from(fln.as_str())
            }
            NFLZError::ValueInNumberedGroupInvalid(fln) => Option::from(fln.as_str()),
            NFLZError::RenameFailed(fln, _, _) => Option::from(fln.as_str()),
            _ => None,
        }
    }
}

impl Display for NFLZError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NFLZError::FilenameMustIncludeExactlyOneNumberedGroup(fln) => f.write_str(&format!(
                "The filename '{}' must include exactly one numbered group.",
                fln
            )),
            NFLZError::ValueInNumberedGroupInvalid(value) => f.write_str(&format!(
                "The value '{}' in the numbered group is not a number.",
                value
            )),
            NFLZError::CantReadDirectory(value, os_err) => f.write_str(&format!(
                "The directory  ('{}') or the files in it can't be read because: {}",
                value, os_err
            )),
            NFLZError::ConflictingFiles(files) => f.write_str(&format!(
                "Can't rename files because {} new file names are in conflict with existing ones.",
                files.len()
            )),
            NFLZError::RenameFailed(old_filename, new_filename, os_err) => f.write_str(&format!(
                "Can't rename file '{}' to '{}' because: {}",
                old_filename, new_filename, os_err,
            )),
            NFLZError::AmbiguousSuffixes(suffixes) => f.write_str(&format!(
                "There are multiple (and therefore ambiguous) suffixes in this directory: {:?}",
                suffixes,
            )),
            NFLZError::AmbiguousPrefixes(prefixes) => f.write_str(&format!(
                "There are multiple (and therefore ambiguous) prefixes in this directory: {:?}",
                prefixes,
            )),
        }
    }
}

impl Error for NFLZError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NFLZError::CantReadDirectory(_, os_err) => Some(os_err),
            _ => None,
        }
    }
}
