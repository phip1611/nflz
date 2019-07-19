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
//! Utility functions to interact with the file system.

use std::collections::btree_map::Values;
use std::fs;
use std::path::Path;

use crate::error::NFLZError;
use crate::nflz::RenameMap;
use crate::parse::ParsedFilename;

/// Reads all matching files from the specified directory with
/// a depth of 0, i.e. it only look for files in subdirectories.
/// * `dir` Directory to search for files
pub fn get_matching_files(dir_path: &Path) -> Result<Vec<ParsedFilename>, NFLZError> {
    let all_files = read_directory_files(dir_path)?;
    let valid_files = all_files
        .into_iter()
        .map(|filename| ParsedFilename::new(filename))
        .filter(|pf| {
            if pf.is_err() {
                let err = pf.as_ref().unwrap_err();
                eprintln!(
                    "NFLZ: skipping file '{}' because: {}",
                    err.filename().unwrap(),
                    err
                );
            }
            pf.is_ok()
        })
        .map(|pf| pf.unwrap())
        .collect::<Vec<ParsedFilename>>();
    Ok(valid_files)
}

/// Reads a directory and returns all paths inside the
/// directory which represent regular files.
fn read_directory_files(dir_path: &Path) -> Result<Vec<String>, NFLZError> {
    let mut filepaths = Vec::new();
    let paths = fs::read_dir(dir_path)
        .map_err(|e| NFLZError::CantReadDirectory(dir_path.to_str().unwrap().to_string(), e))?;

    for path in paths {
        let entry = path.map_err(|e| {
            NFLZError::CantReadDirectory(
                dir_path.file_name().unwrap().to_str().unwrap().to_string(),
                e,
            )
        })?;

        // file_name returns "just" the filename without the full path! that's what we need!
        // for example "paris (12).jpg"
        let filename = entry.file_name();
        let path = entry.path();
        if path.is_file() {
            filepaths.push(filename);
        }
    }

    let filepaths = filepaths
        .into_iter()
        .map(|x| x.to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    Ok(filepaths)
}

/// Returns a list of all files that do exist yet.
pub(crate) fn check_for_existing_files<'a>(
    dir: &Path,
    new_names: Values<'a, ParsedFilename, String>,
) -> Vec<&'a str> {
    let mut existing_files_with_new_name = vec![];
    let iter = new_names.into_iter();
    for new_name in iter {
        let mut path_buf = dir.to_path_buf();
        path_buf.push(new_name);
        let path = path_buf.as_path();
        if Path::new(&path).is_file() {
            existing_files_with_new_name.push(new_name.as_str())
        }
    }
    existing_files_with_new_name
}

/// Renames all files. Make sure to check first if there are conflicts.
pub(crate) fn rename_all_files(rn_map: &RenameMap) -> Result<(), NFLZError> {
    for (old, new) in rn_map {
        fs::rename(old.original_filename(), new).map_err(|io_err| {
            NFLZError::RenameFailed(old.original_filename().to_owned(), new.to_owned(), io_err)
        })?
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_read_directory_files() {
        let dir = std::env::current_dir().unwrap();
        let path = format!("{}/test", dir.as_path().to_str().unwrap());
        let files = read_directory_files(path.as_ref()).unwrap();
        assert_eq!(12, files.len())
    }

    #[test]
    fn test_get_matching_files() {
        let dir = std::env::current_dir().unwrap();
        let path = format!("{}/test", dir.as_path().to_str().unwrap());
        let files = get_matching_files(path.as_ref()).unwrap();
        let found = files
            .iter()
            .map(|pf| pf.original_filename())
            .any(|n| "invalid (100) (19231).jpg" == n);
        assert!(!found);
    }

    #[test]
    fn test_check_for_existing_files() {
        let dir = std::env::current_dir().unwrap();
        let path = format!("{}/test", dir.as_path().to_str().unwrap());

        let mut rn_map = BTreeMap::new();
        rn_map.insert(
            ParsedFilename::new("paris (1).jpg".to_owned()).unwrap(),
            "paris (1).jpg".to_string(),
        );
        rn_map.insert(
            ParsedFilename::new("paris (1414).jpg".to_owned()).unwrap(),
            "paris (1414).jpg".to_string(),
        );
        let values_ref = rn_map.values();
        println!("values_ref: {:#?}", values_ref);
        let existing_files = check_for_existing_files(&Path::new(&path), values_ref);
        assert_eq!(1, existing_files.len());
        assert_eq!("paris (1).jpg", existing_files[0]);
    }
}
