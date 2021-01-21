//! Module related to renaming files.

use crate::error::NFLZError;
use crate::fsutil::check_for_existing_files;
use crate::parse::ParsedFilename;
use std::collections::BTreeMap;
use std::path::Path;

pub type RenameMap = BTreeMap<String, String>;

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
        let value_str_with_leading_zeroes = format!(
            "{}{}",
            String::from("0").repeat(add_digits_count as usize),
            pf.number_group_value()
        );
        let new_filename = format!(
            "{}{}{}",
            pf.filename_prefix(),
            value_str_with_leading_zeroes,
            pf.filename_suffix(),
        );

        // avoid unnecessary renames
        if pf.original_filename() != new_filename {
            map.insert(pf.original_filename().to_string(), new_filename);
        }
    }
    map
}

/// Verifies that all files can be renamed without conflict.
pub fn can_rename_all(dir: &Path, rn_map: &RenameMap) -> Result<(), NFLZError> {
    let new_names_ref = rn_map.values();
    let conflicting_files = check_for_existing_files(dir, new_names_ref);
    if conflicting_files.is_empty() {
        Ok(())
    } else {
        Err(NFLZError::ConflictingFiles(
            conflicting_files.iter().map(|s| s.to_string()).collect(),
        ))
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
    use super::*;
    use crate::fsutil::get_matching_files;

    #[test]
    fn test_compute_rename_map() {
        let dir = std::env::current_dir().unwrap();
        let path = format!("{}/test", dir.as_path().to_str().unwrap());
        let files = get_matching_files(path.as_ref()).unwrap();
        let rn_map = compute_rename_map(&files);

        for i in 1..10 {
            let left_name = format!("paris (00{}).jpg", i);
            let right_name = format!("paris ({}).jpg", i);
            assert_eq!(left_name, *rn_map.get(&right_name).unwrap());

            assert_eq!("paris (010).jpg", rn_map.get("paris (10).jpg").unwrap());
            // no rename necessary
            assert!(!rn_map.contains_key("paris (734).jpg"));
        }
    }

    #[test]
    fn test_can_rename_all() {
        let dir = std::env::current_dir().unwrap();
        let path = format!("{}/test", dir.as_path().to_str().unwrap());
        let files = get_matching_files(path.as_ref()).unwrap();
        let rn_map = compute_rename_map(&files);
        assert!(can_rename_all(path.as_ref(), &rn_map).is_ok());
    }
}
