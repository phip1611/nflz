use std::fs;
use crate::validation;

/// Returns all filenames as strings in the pwd/cwd that matches the pattern that is subject of
/// this programm.
pub fn get_files(pwd: String) -> Vec<String> {
    let mut filepaths = Vec::new();
    let paths = fs::read_dir(pwd).expect("Can't read directory!");
    for path in paths {
        let entry = path.expect("Can't read all paths in this directory!");
        // file_name returns "just" the filename without the full path! that's what we need!
        let filename = entry.file_name().into_string().expect("Can't get filename from path");
        if entry.path().is_file() && validation::filename_is_valid(&filename) {
            filepaths.push(filename);
        }
    }
    filepaths
}