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
//! Module for parsing of file names.

use crate::error::NFLZError;
use crate::math::count_digits_without_leading_zeroes;
use regex::Regex;
use std::cmp::Ordering;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;

/// Represents a file in the filesystem with additional meta-information obtained from the
/// filename relevant for the renaming process.
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// Path to the file.
    path: PathBuf,
    /// The original filename. Obtained by the last component of the `path` field.
    original_filename: String,
    /// The indices at which char the numbered group starts and ends
    /// in the original filename.
    number_group_indices: (u16, u16),
    /// The string inside the filename encapsulated by the indices of field `number_group_indices`.
    /// Might be `"0"`, `"1"`, `"12"`, or `0012`.
    #[allow(unused)]
    number_group_str: String,
    /// Field `number_group_str` parsed as number. Useful for sorting the files.
    number_group_value: u64,
}

impl FileInfo {
    /// Constructor for a new file. Only valid if the file has a filename in the form of
    /// `Img ([0-9]+).jpg` or similar. The constructor does not access the file in the
    /// file system. It relies on that the file actually exists for the lifetime of this struct.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, NFLZError> {
        let filename = path_to_filename(path.as_ref()).to_owned();

        let number_group_indices = get_number_group_indices_from_actual_filename(&filename)?;
        let (from, to) = number_group_indices;
        let number_group_value_str = &filename[from as usize..to as usize];
        let number_group_value = u64::from_str(number_group_value_str).map_err(|_| {
            NFLZError::ValueInNumberedGroupNotANumber(number_group_value_str.to_string())
        })?;

        Ok(Self {
            path: PathBuf::from(path.as_ref()),
            number_group_str: number_group_value_str.to_string(),
            original_filename: filename,
            number_group_indices,
            number_group_value,
        })
    }

    /// Suffix including "(" before the number group inside field [`Self::original_filename`].
    pub fn filename_prefix(&self) -> &str {
        let (prefix, _) =
            get_filename_prefix_and_suffix(self.original_filename(), self.number_group_indices());
        prefix
    }
    /// Prefix including ")" after the number group inside field [`Self::original_filename`].
    pub fn filename_suffix(&self) -> &str {
        let (_, suffix) =
            get_filename_prefix_and_suffix(self.original_filename(), self.number_group_indices());
        suffix
    }
    /// Getter for field `number_group_indices`.
    const fn number_group_indices(&self) -> (u16, u16) {
        self.number_group_indices
    }

    /// Getter for field `number_group_value`.
    pub const fn number_group_value(&self) -> u64 {
        self.number_group_value
    }

    /// Returns the original filename. The filename is obtained by the field `path`.
    /// `/foo/bar/file.ext` => `file.ext`.
    pub fn original_filename(&self) -> &str {
        self.original_filename.as_ref()
    }

    /// Returns the path to the original file.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number_group_value
            .partial_cmp(&other.number_group_value)
    }
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.original_filename() == other.original_filename()
    }
}

impl Eq for FileInfo {}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Wrapper around a [`FileInfo`] that enriches each entry with additional information for a new
/// filename, if the list of all [`FileInfo`] object was processed.
#[derive(Debug, Clone)]
pub struct FileInfoWithRenameAdvice {
    /// See [`FileInfo`].
    file_info: FileInfo,
    /// The new filename with an appropriate number of leading zeroes inside the number group.
    /// This filename includes the value inside the number group with an appropriate amount of
    /// leading zeroes.
    new_filename: Option<String>,
}

impl FileInfoWithRenameAdvice {
    /// Constructor.
    ///
    /// # Parameters
    /// - `file_info`: [`FileInfo`]
    /// - `max_digits`: The maximum amount of digits across all processed  [`FileInfo`] files.
    ///                 For example 4 if the file with the highest number is named `Img (9141).jpg`.
    pub fn new(file_info: FileInfo, max_digits: u64) -> Self {
        assert_ne!(max_digits, 0, "max digits must be bigger than zero");
        let digits = count_digits_without_leading_zeroes(file_info.number_group_value());
        let digits_to_add_count = max_digits - digits;

        if digits_to_add_count == 0 {
            log::debug!(
                "No rename required. File '{}' already has the correct name.",
                file_info.original_filename()
            );
            Self {
                file_info,
                new_filename: None,
            }
        } else {
            // "0001" for example
            let value_str_with_leading_zeros = format!(
                "{}{}",
                String::from("0").repeat(digits_to_add_count as usize),
                file_info.number_group_value()
            );

            // "IMG (001).jpg" for example
            let new_filename = format!(
                "{}{}{}",
                file_info.filename_prefix(),
                value_str_with_leading_zeros,
                file_info.filename_suffix(),
            );

            // should never happen because I have the check for `digits_to_add_count` above
            assert_ne!(
                file_info.original_filename, new_filename,
                "original_filename and new_filename are equal!"
            );

            Self {
                file_info,
                new_filename: Some(new_filename),
            }
        }
    }

    /// Returns true if the file needs a rename, hence, additional
    /// leading zeroes inside the number group.
    pub const fn needs_rename(&self) -> bool {
        self.new_filename.is_some()
    }

    /// Returns true if the file is already properly named.
    /// Opposite of [`Self::needs_rename`].
    pub const fn is_already_properly_named(&self) -> bool {
        self.new_filename.is_none()
    }

    /// Returns the underlying [`FileInfo`].
    pub const fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    /// Returns the path to the new file with respect to [`Self::new_filename`].
    /// None if [`Self::new_filename`] returns `None`.
    pub fn path_with_new_filename(&self) -> Option<PathBuf> {
        self.new_filename.as_ref().map(|new_filename| {
            let mut parent_dir = PathBuf::from(self.file_info.path.parent().unwrap());
            parent_dir.push(new_filename);
            parent_dir
        })
    }

    /// Returns the new filename if the file needs to be renamed. This name includes
    /// the additional leading zeroes inside the number group. If this is None, the
    /// file doesn't need to be renamed. For example, `Img (109).jpg` is already
    /// the correct name if there are no more than `999` files.
    pub fn new_filename(&self) -> Option<&str> {
        self.new_filename.as_deref()
    }

    /// Check if the path returned by [`Self::path_with_new_filename`] already exists, hence,
    /// the rename operation can not continue. Returns always false if [`Self::new_filename`]
    /// is `None`.
    pub fn renamed_file_already_exists(&self) -> bool {
        self.path_with_new_filename()
            .map(|x| x.exists())
            .unwrap_or(false)
    }
}

impl PartialOrd for FileInfoWithRenameAdvice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.file_info.partial_cmp(&other.file_info)
    }
}

impl PartialEq for FileInfoWithRenameAdvice {
    fn eq(&self, other: &Self) -> bool {
        self.file_info.eq(&other.file_info)
    }
}

impl Eq for FileInfoWithRenameAdvice {}

impl Ord for FileInfoWithRenameAdvice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_info.partial_cmp(other.file_info()).unwrap()
    }
}

/// Convenient helper function that transforms a path into the filename.
pub(crate) fn path_to_filename(path: &Path) -> &str {
    match path.components().last().unwrap() {
        Component::Normal(name) => name.to_str().expect("path must be valid utf-8"),
        // if we land here, we received a wrong list of files. Should never happen.
        _ => panic!("Unexpected file path component."),
    }
}

/// Returns either Ok with the indices of the number group or Err. The index
/// doesn't include the parentheses. The first index is inclusive and the last
/// one is exclusive.
/// Example:
/// * `paris (100)` => `Ok((6, 11))` (end is exclusive)
/// * `paris (100) (100)` => `Err()`
fn get_number_group_indices_from_actual_filename(
    actual_filename: &str,
) -> Result<(u16, u16), NFLZError> {
    // let regex = Regex::new(r"(?P<main_group>\([0-9]+\)).*(?P<forbidden_group>\([0-9]+\))?").unwrap();
    let regex = Regex::new(r"(\([0-9]+\))").unwrap();

    // get indices of all matches
    let match_indices = regex
        .find_iter(actual_filename)
        .map(|m| (m.start() as u16, m.end() as u16))
        .collect::<Vec<(u16, u16)>>();

    if match_indices.is_empty() || match_indices.len() > 1 {
        Err(NFLZError::FilenameMustIncludeExactlyOneNumberedGroup(
            actual_filename.to_string(),
        ))
    } else {
        // +-1: remove parentheses
        let from = match_indices[0].0 + 1;
        let to = match_indices[0].1 - 1;
        Ok((from, to))
    }
}

/// Uses the actual filename and the indices obtained by [`get_number_group_indices_from_actual_filename`]
/// to get the prefix before the (...)-group and the suffix behind the (...)-group.
/// * `actual_filename`: Actual filename, like "paris (100)" (without extension).
fn get_filename_prefix_and_suffix(actual_filename: &str, (begin, end): (u16, u16)) -> (&str, &str) {
    (
        &actual_filename[0..begin as usize],
        &actual_filename[end as usize..actual_filename.len()],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number_group_indices_from_actual_filename() {
        let input1 = "img (100)";
        let input2 = "img (1) (100)";
        let input3 = "img (1) 100)";

        let actual1 = get_number_group_indices_from_actual_filename(input1).unwrap();
        assert_eq!(
            5, actual1.0,
            "Number parentheses group starts at index 4 (inclusive)"
        );
        assert_eq!(
            8, actual1.1,
            "Number parentheses group ends at index 9 (exclusive)"
        );

        let actual2 = get_number_group_indices_from_actual_filename(input2);
        assert!(actual2.is_err());

        let actual3 = get_number_group_indices_from_actual_filename(input3).unwrap();
        assert_eq!(
            5, actual3.0,
            "Number parentheses group starts at index 4 (inclusive)"
        );
        assert_eq!(
            6, actual3.1,
            "Number parentheses group ends at index 9 (exclusive)"
        );
    }

    #[test]
    fn test_get_filename_prefix_and_suffix() {
        let input1 = "img (100).jpg";
        let indices1 = get_number_group_indices_from_actual_filename(input1).unwrap();
        let (prefix1, suffix1) = get_filename_prefix_and_suffix(input1, indices1);
        assert_eq!("img (", prefix1);
        assert_eq!(").jpg", suffix1);

        let input2 = "(100) foobar.png";
        let indices2 = get_number_group_indices_from_actual_filename(input2).unwrap();
        let (prefix2, suffix2) = get_filename_prefix_and_suffix(input2, indices2);
        assert_eq!("(", prefix2);
        assert_eq!(") foobar.png", suffix2);
    }

    /*#[test]
    fn test_struct_parsed_filename() {
        let filename1 = "paris (100).png";
        let parsed = NFLZFileInfo::new(filename1.to_owned()).expect("Must be valid");
        assert_eq!((7, 10), parsed.number_group_indices());
        assert_eq!("paris (", parsed.filename_prefix());
        assert_eq!(").png", parsed.filename_suffix());
        assert_eq!(100, parsed.number_group_value());
    }

    #[test]
    fn test_parsed_filename() {
        let p1 = NFLZFileInfo::new("img (1).png".to_string()).unwrap();
        let p1_same = NFLZFileInfo::new("img (1).png".to_string()).unwrap();
        let p2 = NFLZFileInfo::new("img (2).png".to_string()).unwrap();
        assert_eq!(
            p1, p1_same,
            "Two ParsedFilenames are equal if the point to the same original filename."
        );
        assert_ne!(
            p1, p2,
            "Two ParsedFilenames are equal if the point to the same original filename."
        );
        assert!(p1 < p2, "One ParsedFilename is smaller than the other if the number inside the filename is lower.");
    }*/
}
