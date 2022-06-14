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
//! Module related to renaming files.

use crate::error::NFLZError;
use crate::file_info::{FileInfo, FileInfoWithRenameAdvice};
use crate::math::count_digits_without_leading_zeroes;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Main entry point into the library. Helper struct that guides a user through the whole
/// process of the library.
#[derive(Debug)]
pub struct NFLZAssistant {
    /// A copy of the path that was provided by the user.
    path: PathBuf,
    /// Vector with all relevant rename information.
    /// The vector is sorted by the order of numbers inside the filename number groups.
    files_with_rename_info: Vec<FileInfoWithRenameAdvice>,
}

impl NFLZAssistant {
    /// Creates a new object. Needs the working directory where this library should work on.
    /// Not necessarily the present working directory of your shell,
    ///
    /// # Parameters
    /// * `working_dir` Directory to search for files. Expected to be a directory with files in
    ///                 the form `Img (1).jpg`, `Img (2).jpg`, ..., `Img (99).jpg`, ...
    ///                 `Img (124).jpg`.
    pub fn new<P: AsRef<Path>>(working_dir: P) -> Result<Self, NFLZError> {
        // all files inside the directory
        let paths = crate::fsutil::read_directory_flat(working_dir.as_ref()).map_err(|err| {
            NFLZError::CantReadDirectory(PathBuf::from(working_dir.as_ref()), err)
        })?;

        // all valid files that could be parsed
        let files = files_to_nflz_file_info_vec(paths)?;

        let max_digits = find_max_digits(&files);

        let mut files = files
            .into_iter()
            .map(|info| FileInfoWithRenameAdvice::new(info, max_digits))
            .collect::<Vec<_>>();

        // sort by number, ascending
        files.sort();

        Ok(Self {
            path: PathBuf::from(working_dir.as_ref()),
            files_with_rename_info: files,
        })
    }

    /// Verifies that all files can be renamed without conflict.
    /// * `dir` Directory where all replacements happen. Needed to make some checks before the actual renaming starts.
    /// * `rn_map` Map with the mappings from old to new names.
    /// * `pf_list` List with parsed filenames. Needed to make some checks before the actual renaming starts.
    pub fn check_can_rename_all(&self) -> Result<(), NFLZError> {
        check_no_destination_file_already_exists(&self.files_with_rename_info)?;
        check_suffixes_and_prefixes_are_ambiguous(&self.files_with_rename_info)?;
        Ok(())
    }

    /// Renames all files inside the file system if no possible conflicts are detected. Runs
    /// [`Self::check_can_rename_all`] first. Note that there may be external changes to the file
    /// system during that process.
    ///
    /// If the operation is successfully, it returns the same as [`Self::files_to_rename`].
    pub fn rename_all(self) -> Result<Vec<FileInfoWithRenameAdvice>, NFLZError> {
        self.check_can_rename_all()?;
        for file in self.files_to_rename() {
            std::fs::rename(
                file.file_info().path(),
                file.path_with_new_filename()
                    .expect("Must be present at this point! Programming error?!"),
            )
            .map_err(|io_err| {
                NFLZError::RenameFailed(
                    file.file_info().original_filename().to_string(),
                    file.new_filename().unwrap().to_string(),
                    io_err,
                )
            })?;
        }
        Ok(self.files_with_rename_info)
    }

    // GETTERS

    /// Returns all files that need to be renamed. Getter can be used to print
    /// all files that the library is going to change in its final rename operation.
    pub fn files_to_rename(&self) -> Vec<&FileInfoWithRenameAdvice> {
        self.files_with_rename_info
            .iter()
            .filter(|new_filename| new_filename.needs_rename())
            .collect()
    }

    /// Returns all files that need to be renamed because their file name already
    /// fits into the order of the other files. Getter can be used to print all files
    /// that the library will not change during its final rename operation.
    pub fn files_without_rename(&self) -> Vec<&FileInfoWithRenameAdvice> {
        self.files_with_rename_info
            .iter()
            .filter(|new_filename| new_filename.is_already_properly_named())
            .collect()
    }

    /// Returns a copy of the original user input path.
    pub const fn path(&self) -> &PathBuf {
        &self.path
    }
}

/// Transforms all files by their path to a list of [`FileInfo`]. Files that can't be parsed
/// to [`FileInfo`] are skipped. Thus, files such as `.gitignore` etc do not hinder the library.
fn files_to_nflz_file_info_vec(paths: Vec<PathBuf>) -> Result<Vec<FileInfo>, NFLZError> {
    let mut vec = Vec::with_capacity(paths.len());
    for path in paths {
        let file = FileInfo::new(path);
        match file {
            Ok(file) => {
                vec.push(file);
            }
            Err(err) => {
                match err {
                    // this is acceptable; skip irrelevant files
                    NFLZError::FilenameMustIncludeExactlyOneNumberedGroup(filename) => {
                        log::info!("Skipping file '{filename}'");
                        continue;
                    }
                    NFLZError::ValueInNumberedGroupNotANumber(filename) => {
                        log::warn!("Skipping file '{filename}' because of invalid number within number group.");
                        continue;
                    }
                    _ => (),
                }
                // if the previous match didn't execute the "continue", we have a hard error
                // => return early from function
                return Err(err);
            }
        }
    }
    Ok(vec)
}

/// Searches all files and returns the highest count of digits in a number in a number group.
fn find_max_digits(files: &[FileInfo]) -> u64 {
    let nums = files
        .iter()
        .map(|pf| pf.number_group_value())
        .collect::<Vec<u64>>();
    let max = nums.into_iter().max().unwrap_or(0);
    count_digits_without_leading_zeroes(max)
}

fn check_no_destination_file_already_exists(
    files: &[FileInfoWithRenameAdvice],
) -> Result<(), NFLZError> {
    let files = files
        .iter()
        .filter(|file| file.renamed_file_already_exists())
        .collect::<Vec<_>>();
    if files.is_empty() {
        Ok(())
    } else {
        let paths = files
            .iter()
            .map(|info| PathBuf::from(info.file_info().path()))
            .collect::<Vec<_>>();
        Err(NFLZError::ConflictingFiles(paths))
    }
}

#[allow(non_snake_case)]
fn check_suffixes_and_prefixes_are_ambiguous(
    pf_list: &[FileInfoWithRenameAdvice],
) -> Result<(), NFLZError> {
    let mut prefix_set = HashSet::new();
    let mut suffix_set = HashSet::new();

    for pf in pf_list {
        prefix_set.insert(pf.file_info().filename_prefix());
        suffix_set.insert(pf.file_info().filename_suffix());
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

#[cfg(test)]
mod tests {
    use crate::NFLZAssistant;

    #[test]
    fn test_nflz() {
        let assistant = NFLZAssistant::new("./test-resources").unwrap();
        let files_to_rename = assistant.files_to_rename();
        let files_without_rename = assistant.files_without_rename();
        assert_eq!(
            files_without_rename.len() + files_to_rename.len(),
            11,
            "must skip file invalid file that doesn't match the pattern!"
        );

        let actual = files_to_rename
            .iter()
            .map(|f| f.new_filename().expect("must be available at this point"))
            .collect::<Vec<_>>();
        assert_eq!(
            [
                "paris (001).jpg",
                "paris (002).jpg",
                "paris (003).jpg",
                "paris (004).jpg",
                "paris (005).jpg",
                "paris (006).jpg",
                "paris (007).jpg",
                "paris (008).jpg",
                "paris (009).jpg",
                "paris (010).jpg",
            ],
            actual.as_slice()
        );

        let actual = files_without_rename
            .iter()
            .map(|f| f.file_info().original_filename())
            .collect::<Vec<_>>();
        assert_eq!(["paris (734).jpg"], actual.as_slice());

        assert!(assistant.check_can_rename_all().is_ok());
    }
}
