/*
MIT License

Copyright (c) 2022 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
//! Module for errors inside NFLZ library. See [`NFLZError`].

use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

/// Main error of the library.
#[derive(Debug)]
pub enum NFLZError {
    /// File names must include at least one numbered group.
    /// Example: "Img (1).jpg" is valid but "Img (2) (4).jpg" is not.
    FilenameMustIncludeExactlyOneNumberedGroup(String),
    /// The value inside the group must be a valid number.
    ValueInNumberedGroupNotANumber(String),
    /// Can't read the specified directory,
    CantReadDirectory(PathBuf, std::io::Error),
    /// There are files that would have the same filename in the end.
    /// Would overwrite files.
    ConflictingFiles(Vec<PathBuf>),
    /// The renaming failed.
    RenameFailed(String, String, std::io::Error),
    /// The prefixes of all files inside the directory before the rename group
    /// must be unambiguous. Hence, "Img (1).jpg" and "Photo (2).jpg" will result in an error.
    AmbiguousPrefixes(HashSet<String>),
    /// The suffixes of all files inside the directory after the rename group
    /// must be unambiguous. Hence, "Img (1) foobar.jpg" and "Img (1) barfoo.png" will result
    /// in an error.
    AmbiguousSuffixes(HashSet<String>),
}

impl NFLZError {
    /// The filename that resulted in an error.
    pub fn filename(&self) -> Option<&str> {
        match self {
            Self::FilenameMustIncludeExactlyOneNumberedGroup(fln) => Option::from(fln.as_str()),
            Self::ValueInNumberedGroupNotANumber(fln) => Option::from(fln.as_str()),
            Self::RenameFailed(fln, _, _) => Option::from(fln.as_str()),
            _ => None,
        }
    }
}

impl Display for NFLZError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FilenameMustIncludeExactlyOneNumberedGroup(fln) => f.write_str(&format!(
                "The filename '{}' must include exactly one numbered group.",
                fln
            )),
            Self::ValueInNumberedGroupNotANumber(value) => f.write_str(&format!(
                "The value '{}' in the numbered group is not a number.",
                value
            )),
            Self::CantReadDirectory(value, os_err) => f.write_str(&format!(
                "The directory  ('{}') or the files in it can't be read because: {}",
                value.as_os_str().to_str().unwrap(),
                os_err
            )),
            Self::ConflictingFiles(files) => f.write_str(&format!(
                "Can't rename files because {} new file names are in conflict with existing ones.",
                files.len()
            )),
            Self::RenameFailed(old_filename, new_filename, os_err) => f.write_str(&format!(
                "Can't rename file '{}' to '{}' because: {}",
                old_filename, new_filename, os_err,
            )),
            Self::AmbiguousSuffixes(suffixes) => f.write_str(&format!(
                "There are multiple (and therefore ambiguous) suffixes in this directory: {:?}",
                suffixes,
            )),
            Self::AmbiguousPrefixes(prefixes) => f.write_str(&format!(
                "There are multiple (and therefore ambiguous) prefixes in this directory: {:?}",
                prefixes,
            )),
        }
    }
}

impl Error for NFLZError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CantReadDirectory(_, os_err) => Some(os_err),
            _ => None,
        }
    }
}
