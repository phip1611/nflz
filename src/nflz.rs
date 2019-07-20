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

/// Returns a map from the old filename to the new filenames; it will skip files where no action
/// is needed.
pub fn get_new_filename_map<'a>(rename_map: &HashMap<&'a String, TransformationInformation>, max_digits: usize) -> HashMap<&'a String, String> {
    let mut map = HashMap::new();
    let mut skipped_any = false;
    for (k, v) in rename_map.iter() {
        let new_filename = map_filename(k, v, max_digits);

        // btw: need to compare the strings because we can't compare the number with max digits
        // because we have the number in TransformationInformation not in the string format
        // ==> zeros are stripped

        // Strings equals? (e.g. is nflz is running again in the same directory)
        if **k == new_filename {
            skipped_any = true;
            eprintln!("No action needed for file '{}'; will skip", k);
        } else {
            map.insert(*k, new_filename);
        }
    }

    if skipped_any {
        println!(); // newline; then "nlfz will rename the ..." is in next line
    }

    let map = map;
    map
}

/// Transform the filename-string into the string with leading zeros in the (...)-group
fn map_filename(name: &String, info: &TransformationInformation, digits: usize) -> String {
    let mut new_filename = String::from(&name[0 .. info.indices.start + 1]); // + 1 to include '('
    let digits_current = math_util::digits(&info.number);
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

/// Shows the user all files that are going to be renamed
pub fn show_user_intended_actions(map: &HashMap<&String, String>) {
    if map.len() == 0 {
        // this will happen if there are files in the directory but they are already
        // ALL renamed
        println!("No files to rename left; will exit");
        return;
    } else {
        println!("nlfz will rename the following files:\n");
    }

    let mut longest_name = 0;

    // we need the longest name so that we can add spaces
    // to the key beeing printed; so we get something like this
    // stdout:
    //  paris (2).txt   => paris (2).txt
    //  paris (1).txt => paris (1).txt
    // rather than
    //  paris (2).txt => paris (2).txt
    //  paris (1).txt => paris (1).txt

    for (k, _v) in map.iter() {
        if k.len() > longest_name {
            longest_name = k.len();
        }
    }

    for (k, v) in map.iter() {
        let mut x = String::from(*k);
        for _i in 0 .. longest_name - k.len() {
            x.push(' ');
        }
        println!("{} => {}", x, v);
    }
}
