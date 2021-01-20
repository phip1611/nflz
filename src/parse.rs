//! Module for parsing of file names.

use crate::error::NFLZError;
use regex::Regex;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Represents a parsed filename that matches
/// the contract of this library, i.e. only files
/// that can be renamed successfully.
#[derive(Debug)]
pub struct ParsedFilename {
    original_filename: String,
    number_group_indices: (u16, u16),
    number_group_value: u64,
}

impl ParsedFilename {
    pub fn new(full_filename: String) -> Result<Self, NFLZError> {
        let (actual_filename, _extension) = split_filename(&full_filename);
        let number_group_indices = get_number_group_indices_from_actual_filename(actual_filename)?;
        let from = number_group_indices.0 as usize + 1; // remove parentheses
        let to = number_group_indices.1 as usize - 1; // remove parentheses
        let number_group_value_str = &actual_filename[from .. to];
        let number_group_value = u64::from_str(number_group_value_str)
            .map_err(|_| NFLZError::ValueInNumberedGroupInvalid(number_group_value_str.to_string()))?;

        Ok(ParsedFilename {
            original_filename: full_filename,
            number_group_indices,
            number_group_value,
        })
    }

    /// Getter for field `original_filename`.
    pub fn original_filename(&self) -> &str {
        &self.original_filename
    }
    /// Getter for field `actual_filename`.
    pub fn actual_filename(&self) -> &str {
        let (actual_filename, _) = split_filename(self.original_filename());
        actual_filename
    }
    /// Getter for field `actual_filename_prefix`.
    pub fn actual_filename_prefix(&self) -> &str {
        let (prefix, _) =
            get_filename_prefix_and_suffix(self.actual_filename(), self.number_group_indices());
        prefix
    }
    /// Getter for field `actual_filename_suffix`.
    pub fn actual_filename_suffix(&self) -> &str {
        let (_, suffix) =
            get_filename_prefix_and_suffix(self.actual_filename(), self.number_group_indices());
        suffix
    }
    /// Getter for field `extension`.
    pub fn extension(&self) -> &str {
        let (_, extension) = split_filename(self.original_filename());
        extension
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
        f.write_str(&format!("ParsedFilename({})", self.original_filename))
    }
}

/// Splits a complete filename into the actual filename
/// and the extension. `"img (1).jpg"` will become `["img (1)", "jpg"]`
/// whereas `"img (1).foobar.jpg"` will become `["img (1)", "foobar.jpg"]`
fn split_filename(filename: &str) -> (&str, &str) {
    let index = filename
        .find('.')
        .expect(&format!("Input ('{}') must be a filename with an extension!", filename));
    // +1 to skip leading "."
    (&filename[0..index], &filename[index + 1..filename.len()])
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
        Ok(match_indices[0])
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
    fn test_split_filename() {
        let input1 = "img (1).jpg";
        let input2 = "img (1).foobar.jpg";

        let expected_actual_filename_1 = "img (1)";
        let expected_extension_1 = "jpg";
        let expected_actual_filename_2 = "img (1)";
        let expected_extension_2 = "foobar.jpg";

        let (actual_actual_filename_1, actual_extension_1) = split_filename(input1);
        let (actual_actual_filename_2, actual_extension_2) = split_filename(input2);

        assert_eq!(expected_actual_filename_1, actual_actual_filename_1);
        assert_eq!(expected_actual_filename_2, actual_actual_filename_2);
        assert_eq!(expected_extension_1, actual_extension_1);
        assert_eq!(expected_extension_2, actual_extension_2);
    }

    #[test]
    fn test_get_number_group_indices_from_actual_filename() {
        let input1 = "img (100)";
        let input2 = "img (1) (100)";
        let input3 = "img (1) 100)";

        let actual1 = get_number_group_indices_from_actual_filename(input1).unwrap();
        assert_eq!(
            4, actual1.0,
            "Number parentheses group starts at index 4 (inclusive)"
        );
        assert_eq!(
            9, actual1.1,
            "Number parentheses group ends at index 9 (exclusive)"
        );

        let actual2 = get_number_group_indices_from_actual_filename(input2);
        assert!(actual2.is_err());

        let actual3 = get_number_group_indices_from_actual_filename(input3).unwrap();
        assert_eq!(
            4, actual3.0,
            "Number parentheses group starts at index 4 (inclusive)"
        );
        assert_eq!(
            7, actual3.1,
            "Number parentheses group ends at index 9 (exclusive)"
        );
    }

    #[test]
    fn test_get_filename_prefix_and_suffix() {
        let input1 = "img (100)";
        let indices1 = get_number_group_indices_from_actual_filename(input1).unwrap();
        let (prefix1, suffix1) = get_filename_prefix_and_suffix(input1, indices1);
        assert_eq!("img ", prefix1);
        assert_eq!("", suffix1);

        let input2 = "(100) foobar";
        let indices2 = get_number_group_indices_from_actual_filename(input2).unwrap();
        let (prefix2, suffix2) = get_filename_prefix_and_suffix(input2, indices2);
        assert_eq!("", prefix2);
        assert_eq!(" foobar", suffix2);
    }

    #[test]
    fn test_struct_parsed_filename() {
        let filename1 = "paris (100).png";
        let parsed = ParsedFilename::new(filename1.to_owned()).expect("Must be valid");
        assert_eq!((6, 11), parsed.number_group_indices());
        assert_eq!("paris (100)", parsed.actual_filename());
        assert_eq!("png", parsed.extension());
        assert_eq!("paris ", parsed.actual_filename_prefix());
        assert_eq!("", parsed.actual_filename_suffix());
        assert_eq!(100, parsed.number_group_value());
    }
}
