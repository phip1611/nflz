/*
MIT License

Copyright (c) 2021 Philipp Schuster

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

use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use regex::Regex;

use crate::error::NFLZError;

/// Represents a parsed filename that matches
/// the contract of this library, i.e. only files
/// that can be renamed successfully.
#[derive(Debug, Clone)]
pub struct ParsedFilename {
    original_filename: String,
    number_group_indices: (u16, u16),
    number_group_value: u64,
}

impl PartialOrd for ParsedFilename {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.number_group_value() > other.number_group_value() {
            Ordering::Greater
        } else if self.number_group_value() == other.number_group_value() {
            Ordering::Equal
        } else {
            Ordering::Less
        })
    }
}

impl PartialEq for ParsedFilename {
    fn eq(&self, other: &Self) -> bool {
        self.original_filename() == other.original_filename()
    }
}

impl Eq for ParsedFilename {}

impl Ord for ParsedFilename {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl ParsedFilename {
    pub fn new(original_filename: String) -> Result<Self, NFLZError> {
        let number_group_indices =
            get_number_group_indices_from_actual_filename(&original_filename)?;
        let (from, to) = number_group_indices;
        let number_group_value_str = &original_filename[from as usize..to as usize];
        let number_group_value = u64::from_str(number_group_value_str).map_err(|_| {
            NFLZError::ValueInNumberedGroupInvalid(number_group_value_str.to_string())
        })?;

        Ok(ParsedFilename {
            original_filename,
            number_group_indices,
            number_group_value,
        })
    }

    /// Getter for field `original_filename`.
    pub fn original_filename(&self) -> &str {
        &self.original_filename
    }

    /// Suffix including "(" before the number group inside field [`original_filename`].
    pub fn filename_prefix(&self) -> &str {
        let (prefix, _) =
            get_filename_prefix_and_suffix(self.original_filename(), self.number_group_indices());
        prefix
    }
    /// Prefix including ")" after the number group inside field [`original_filename`].
    pub fn filename_suffix(&self) -> &str {
        let (_, suffix) =
            get_filename_prefix_and_suffix(self.original_filename(), self.number_group_indices());
        suffix
    }
    /// Getter for field `number_group_indices`.
    pub fn number_group_indices(&self) -> (u16, u16) {
        self.number_group_indices
    }
    /// Getter for field `number_group_value`.
    pub fn number_group_value(&self) -> u64 {
        self.number_group_value
    }
}

impl Display for ParsedFilename {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&format!("{}", self.original_filename))
    }
}

/// Returns either Ok with the indices of the number group or Err. The index
/// includes the parentheses. The first index is inclusive and the last one is exclusive.
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

    #[test]
    fn test_struct_parsed_filename() {
        let filename1 = "paris (100).png";
        let parsed = ParsedFilename::new(filename1.to_owned()).expect("Must be valid");
        assert_eq!((7, 10), parsed.number_group_indices());
        assert_eq!("paris (", parsed.filename_prefix());
        assert_eq!(").png", parsed.filename_suffix());
        assert_eq!(100, parsed.number_group_value());
    }

    #[test]
    fn test_parsed_filename() {
        let p1 = ParsedFilename::new("img (1).png".to_string()).unwrap();
        let p1_same = ParsedFilename::new("img (1).png".to_string()).unwrap();
        let p2 = ParsedFilename::new("img (2).png".to_string()).unwrap();
        assert_eq!(
            p1, p1_same,
            "Two ParsedFilenames are equal if the point to the same original filename."
        );
        assert_ne!(
            p1, p2,
            "Two ParsedFilenames are equal if the point to the same original filename."
        );
        assert!(p1 < p2, "One ParsedFilename is smaller than the other if the number inside the filename is lower.");
    }
}
