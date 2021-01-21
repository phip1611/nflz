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
//! Module related to renaming files.

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use crate::error::NFLZError;
use crate::fsutil::check_for_existing_files;
use crate::parse::ParsedFilename;

pub type RenameMap = BTreeMap<ParsedFilename, String>;

/// Compute the rename map. This is a mapping from original file name
/// to the name it would rename the file in the next step.
/// It avoids unnecessary renames (oldname == newname).
pub fn compute_rename_map(pf_list: &Vec<ParsedFilename>) -> RenameMap {
    let mut map = BTreeMap::new();
    if pf_list.is_empty() {
        return map;
    }

    let nums = pf_list
        .iter()
        .map(|pf| pf.number_group_value())
        .collect::<Vec<u64>>();
    let max = nums.iter().max().unwrap();
    let max_digits = digits(*max);
    for pf in pf_list {
        let digits = digits(pf.number_group_value());
        let add_digits_count = max_digits - digits;
        let value_str_with_leading_zeros = format!(
            "{}{}",
            String::from("0").repeat(add_digits_count as usize),
            pf.number_group_value()
        );
        let new_filename = format!(
            "{}{}{}",
            pf.filename_prefix(),
            value_str_with_leading_zeros,
            pf.filename_suffix(),
        );

        // avoid unnecessary renames
        if pf.original_filename() != new_filename {
            map.insert(pf.clone(), new_filename);
        }
    }
    map
}

/// Verifies that all files can be renamed without conflict.
pub fn can_rename_all(
    dir: &Path,
    rn_map: &RenameMap,
    pf_list: &Vec<ParsedFilename>,
) -> Result<(), NFLZError> {
    can_rename_all__destination_files(dir, rn_map)?;
    can_rename_all__same_suffix_and_prefix(pf_list)?;
    Ok(())
}

/// Renames all files according to the mappings in the rename map
/// if [`can_rename_all`] returns `Ok`.
pub fn rename_all(
    dir: &Path,
    rn_map: &RenameMap,
    pf_list: &Vec<ParsedFilename>,
) -> Result<(), NFLZError> {
    can_rename_all(dir, rn_map, pf_list)?;
    crate::fsutil::rename_all_files(&rn_map)?;
    Ok(())
}

#[allow(non_snake_case)]
fn can_rename_all__destination_files(dir: &Path, rn_map: &RenameMap) -> Result<(), NFLZError> {
    let new_names_ref = rn_map.values();
    let conflicting_files = check_for_existing_files(dir, new_names_ref);

    // check that now file with one of the new names already exists
    if !conflicting_files.is_empty() {
        Err(NFLZError::ConflictingFiles(
            conflicting_files.iter().map(|s| s.to_string()).collect(),
        ))
    } else {
        Ok(())
    }
}

#[allow(non_snake_case)]
fn can_rename_all__same_suffix_and_prefix(pf_list: &Vec<ParsedFilename>) -> Result<(), NFLZError> {
    let mut prefix_set = HashSet::new();
    let mut suffix_set = HashSet::new();

    for pf in pf_list {
        prefix_set.insert(pf.filename_prefix());
        suffix_set.insert(pf.filename_suffix());
    }

    if prefix_set.len() > 1 {
        Err(NFLZError::AmbiguousPrefixes(
            prefix_set
                .into_iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
        ))
    } else if suffix_set.len() > 1 {
        Err(NFLZError::AmbiguousSuffixes(
            suffix_set
                .into_iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
        ))
    } else {
        Ok(())
    }
}

/// Returns the amount of digits of a number.
/// For example: 12345 => 5
fn digits(number: u64) -> u32 {
    let x = (number + 1) as f64;
    x.log10().ceil() as u32
}

#[cfg(test)]
mod tests {
    use crate::fsutil::get_matching_files;

    use super::*;

    #[test]
    fn test_compute_rename_map() {
        let dir = std::env::current_dir().unwrap();
        let path = format!("{}/test", dir.as_path().to_str().unwrap());
        let files = get_matching_files(path.as_ref()).unwrap();
        let rn_map = compute_rename_map(&files);

        for i in 1..10 {
            let expected = format!("paris (00{}).jpg", i);
            let input = ParsedFilename::new(format!("paris ({}).jpg", i)).unwrap();
            assert_eq!(expected, *rn_map.get(&input).unwrap());
        }

        let expected_paris10 = ParsedFilename::new("paris (10).jpg".to_string()).unwrap();

        assert_eq!("paris (010).jpg", *rn_map.get(&expected_paris10).unwrap());

        // no rename necessary

        let not_expected_paris734 = ParsedFilename::new("paris (734).jpg".to_string()).unwrap();
        assert!(!rn_map.contains_key(&not_expected_paris734));
    }

    #[test]
    fn test_can_rename_all() {
        let dir = std::env::current_dir().unwrap();
        let path = format!("{}/test", dir.as_path().to_str().unwrap());
        let files = get_matching_files(path.as_ref()).unwrap();
        let rn_map = compute_rename_map(&files);
        assert!(can_rename_all(path.as_ref(), &rn_map, &files).is_ok());
    }
}
