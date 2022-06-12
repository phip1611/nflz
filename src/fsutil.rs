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
//! Utility functions to interact with the file system. Main function of this module is
//! [`read_directory_flat`].

use std::fs;
use std::path::{Path, PathBuf};

/// Reads all matching files for the purpose of this library from the specified directory. The
/// search depth is 0, i.e., the function doesn't look for files in subdirectories.
///
/// # Parameters
/// * `dir_path` Directory to search for files. Expected to be a directory with files in the form
///              `Img (1).jpg`, `Img (2).jpg`, ..., `Img (99).jpg`, ... `Img (124).jpg`.
///
/// # Return Type
/// The returned type is a vector of [`PathBuf`].
pub fn read_directory_flat<P: AsRef<Path>>(dir_path: P) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    let dir_handle = fs::read_dir(dir_path)?;

    for path in dir_handle {
        // errors only if during the process the file system gets changed or a
        // similar weird situation occurs
        let entry = path?;

        let typ = entry.file_type()?;

        if !typ.is_file() {
            break;
        }

        files.push(entry.path())
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_info::path_to_filename;

    #[test]
    fn test_read_directory_flat() {
        let files = read_directory_flat("./test-resources").unwrap();
        assert_eq!(12, files.len());
        let mut expected = vec![
            "invalid (100) (19231).jpg",
            "paris (1).jpg",
            "paris (2).jpg",
            "paris (3).jpg",
            "paris (4).jpg",
            "paris (5).jpg",
            "paris (6).jpg",
            "paris (7).jpg",
            "paris (8).jpg",
            "paris (9).jpg",
            "paris (10).jpg",
            "paris (734).jpg",
        ];
        expected.sort();
        let mut actual = files
            .iter()
            .map(|path| path_to_filename(path))
            .collect::<Vec<_>>();
        actual.sort();
        assert_eq!(actual.as_slice(), expected);
    }
}
