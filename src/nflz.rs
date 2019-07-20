use std::collections::HashMap;
use std::fs;
use crate::math_util;

/// Struct that describes the indices in the filename where the (...)-group is find
#[derive(Debug)]
struct TransformationIndicesInformation {
    pub start: usize,
    pub end: usize
}

impl TransformationIndicesInformation {
    /// Constructs a new TransformationIndicesInformation
    fn from(start: usize, end: usize) -> TransformationIndicesInformation {
        TransformationIndicesInformation {
            start,
            end
        }
    }
}

/// truct that describes all information needed for the transformation/renaming.
#[derive(Debug)]
pub struct TransformationInformation {
    number: usize,
    indices: TransformationIndicesInformation
}

impl TransformationInformation {
    /// COnstructs a new TransformationInformation
    pub fn new(number: usize, index_start: usize, index_end: usize) -> TransformationInformation {
        TransformationInformation {
            number,
            indices: TransformationIndicesInformation::from(
                index_start,
                index_end
            )
        }
    }
}

/// Returns a map from the old filename to the new filenames
pub fn get_new_filename_map<'a>(rename_map: &HashMap<&'a String, TransformationInformation>, digits: usize) -> HashMap<&'a String, String> {
    let mut map = HashMap::new();
    for (k, v) in rename_map.iter() {
        map.insert(*k, map_filename(k, v, digits));
    }
    let map = map;
    map
}

/// Transform the filename-string into the string with leading zeros in the (...)-group
fn map_filename(name: &String, info: &TransformationInformation, digits: usize) -> String {
    let mut new_filename = String::from(&name[0 .. info.indices.start + 1]); // + 1 to include '('
    let digits_current = math_util::digits(info.number);
    for _i in 0 .. (digits - digits_current) {
        new_filename.push('0');
    }
    new_filename.push_str(&info.number.to_string());

    new_filename.push_str(&name[info.indices.end - 1 .. name.len()]); // - 1 to include ')'

    let new_filename = new_filename;
    new_filename
}

/// Renames all files in the filesystem
pub fn rename_all_files(map: HashMap<&String, String>) {
    for (k, v) in map.iter() {
        fs::rename(k, v).expect(&format!("Could not rename file {} to {}", k, v)); // Rename a.txt to b.txt
    }
}