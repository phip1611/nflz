mod parse;
mod error;
mod fsutil;
mod rename;


use std::path::Path;
use crate::rename::{RenameMap, compute_rename_map};
pub use crate::error::NFLZError;

/// Function to verify all renames from [`get_renameable_files`] without renaming it.
pub use rename::can_rename_all;
/// Actually reanems all files from [`get_renameable_files`]. Make sure to call
/// [`can_rename_all`] first.
pub use fsutil::rename_all;

/// Returns all renameable files in the given directory.
pub fn get_renamable_files(dir: &Path) -> Result<RenameMap, NFLZError> {
    let files = fsutil::get_matching_files(dir)?;
    Ok(compute_rename_map(&files))
}

